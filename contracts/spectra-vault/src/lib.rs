#![no_std]

pub mod errors;
pub mod events;
pub mod math;
pub mod storage;
pub mod types;

#[cfg(test)]
mod test;

use crate::errors::ContractError;
use crate::storage::DataKey;
use crate::types::{AssetAllocation, RebalanceRecord, VaultConfig, VaultStatus};
use soroban_sdk::{
    contract, contractimpl, token, Address, BytesN, Env, String, Vec,
};

#[contract]
pub struct SpectraVault;

#[contractimpl]
impl SpectraVault {
    /// Create a new asset basket vault.
    pub fn create_vault(
        env: Env,
        vault_id: BytesN<32>,
        name: String,
        metadata_hash: BytesN<32>,
        manager: Address,
        base_asset: Address,
        rebalance_authority: Address,
        management_fee_bps: u32,
        allocations: Vec<AssetAllocation>,
    ) -> Result<(), ContractError> {
        if env.storage().persistent().has(&DataKey::VaultConfig(vault_id.clone())) {
            return Err(ContractError::DuplicateVault);
        }

        if management_fee_bps > 500 { // Max 5% fee cap
            return Err(ContractError::FeeTooHigh);
        }

        Self::validate_allocations(&allocations)?;

        let config = VaultConfig {
            name,
            metadata_hash,
            manager: manager.clone(),
            base_asset,
            rebalance_authority,
            management_fee_bps,
            deposit_status: VaultStatus::Active,
            withdrawal_status: VaultStatus::Active,
        };

        env.storage().persistent().set(&DataKey::VaultConfig(vault_id.clone()), &config);
        env.storage().persistent().set(&DataKey::VaultAllocations(vault_id.clone()), &allocations);
        env.storage().persistent().set(&DataKey::VaultTotalShares(vault_id.clone()), &0i128);

        events::emit_vault_created(&env, vault_id, manager);

        Ok(())
    }

    /// Deposit a supported asset into a vault.
    pub fn deposit(
        env: Env,
        vault_id: BytesN<32>,
        user: Address,
        asset: Address,
        amount: i128,
    ) -> Result<i128, ContractError> {
        user.require_auth();

        if amount <= 0 {
            return Err(ContractError::ZeroAmount);
        }

        let mut config = Self::get_config(&env, &vault_id)?;
        if config.deposit_status == VaultStatus::Paused {
            return Err(ContractError::DepositsPaused);
        }

        let allocations = Self::get_allocations(&env, &vault_id)?;
        let mut is_supported = false;
        for alloc in allocations.iter() {
            if alloc.asset == asset {
                is_supported = true;
                break;
            }
        }
        if !is_supported {
            return Err(ContractError::InvalidAsset);
        }

        // Calculate fees if any
        let fee_amount = if config.management_fee_bps > 0 {
            math::calculate_fee(amount, config.management_fee_bps)
        } else {
            0
        };
        let net_amount = amount - fee_amount;

        let total_shares = Self::get_total_shares(&env, &vault_id);
        let total_vault_value = Self::calculate_total_vault_value(&env, &vault_id, &allocations)?;

        // Calculate shares to mint based on net amount
        let shares_to_mint = math::calculate_shares_to_mint(net_amount, total_shares, total_vault_value);

        // Transfer asset from user to contract
        let token_client = token::Client::new(&env, &asset);
        token_client.transfer(&user, &env.current_contract_address(), &amount);

        // Update accounting
        let new_total_shares = total_shares + shares_to_mint;
        env.storage().persistent().set(&DataKey::VaultTotalShares(vault_id.clone()), &new_total_shares);

        let user_shares = Self::get_user_shares(&env, &vault_id, &user);
        env.storage().persistent().set(&DataKey::VaultUserShares(vault_id.clone(), user.clone()), &(user_shares + shares_to_mint));

        let asset_balance = Self::get_asset_balance(&env, &vault_id, &asset);
        env.storage().persistent().set(&DataKey::VaultAssetBalance(vault_id.clone(), asset.clone()), &(asset_balance + net_amount));

        if fee_amount > 0 {
            let accrued_fees = Self::get_accrued_fees(&env, &vault_id, &asset);
            env.storage().persistent().set(&DataKey::VaultAccruedFees(vault_id.clone(), asset.clone()), &(accrued_fees + fee_amount));
        }

        events::emit_deposit(&env, vault_id, user, asset, net_amount, shares_to_mint);

        Ok(shares_to_mint)
    }

    /// Withdraw from a vault by redeeming shares.
    pub fn withdraw(
        env: Env,
        vault_id: BytesN<32>,
        user: Address,
        shares_to_burn: i128,
    ) -> Result<Vec<(Address, i128)>, ContractError> {
        user.require_auth();

        if shares_to_burn <= 0 {
            return Err(ContractError::ZeroAmount);
        }

        let config = Self::get_config(&env, &vault_id)?;
        if config.withdrawal_status == VaultStatus::Paused {
            return Err(ContractError::WithdrawalsPaused);
        }

        let user_shares = Self::get_user_shares(&env, &vault_id, &user);
        if user_shares < shares_to_burn {
            return Err(ContractError::InsufficientShares);
        }

        let total_shares = Self::get_total_shares(&env, &vault_id);
        let allocations = Self::get_allocations(&env, &vault_id)?;
        let mut withdrawn_amounts = Vec::new(&env);

        for alloc in allocations.iter() {
            let asset_balance = Self::get_asset_balance(&env, &vault_id, &alloc.asset);
            let amount_to_withdraw = math::calculate_proportional_amount(shares_to_burn, total_shares, asset_balance);
            
            if amount_to_withdraw > 0 {
                let token_client = token::Client::new(&env, &alloc.asset);
                token_client.transfer(&env.current_contract_address(), &user, &amount_to_withdraw);
                
                env.storage().persistent().set(
                    &DataKey::VaultAssetBalance(vault_id.clone(), alloc.asset.clone()),
                    &(asset_balance - amount_to_withdraw)
                );
                withdrawn_amounts.push_back((alloc.asset, amount_to_withdraw));
            }
        }

        // Update accounting
        let new_total_shares = total_shares - shares_to_burn;
        env.storage().persistent().set(&DataKey::VaultTotalShares(vault_id.clone()), &new_total_shares);
        env.storage().persistent().set(&DataKey::VaultUserShares(vault_id.clone(), user.clone()), &(user_shares - shares_to_burn));

        events::emit_withdrawal(&env, vault_id, user, shares_to_burn, withdrawn_amounts.clone());

        Ok(withdrawn_amounts)
    }

    /// Update target allocations (Permissioned).
    pub fn update_allocations(
        env: Env,
        vault_id: BytesN<32>,
        new_allocations: Vec<AssetAllocation>,
    ) -> Result<(), ContractError> {
        let mut config = Self::get_config(&env, &vault_id)?;
        config.rebalance_authority.require_auth();

        Self::validate_allocations(&new_allocations)?;

        let old_allocations = Self::get_allocations(&env, &vault_id)?;
        
        // In a real scenario, we'd hash the allocations. For simplicity, we'll use empty hashes or simplified logic.
        let old_hash = BytesN::from_array(&env, &[0u8; 32]); 
        let new_hash = BytesN::from_array(&env, &[1u8; 32]);

        let record = RebalanceRecord {
            timestamp: env.ledger().timestamp(),
            old_allocation_hash: old_hash.clone(),
            new_allocation_hash: new_hash.clone(),
        };

        let mut history: Vec<RebalanceRecord> = env.storage().persistent()
            .get(&DataKey::VaultRebalanceHistory(vault_id.clone()))
            .unwrap_or(Vec::new(&env));
        history.push_back(record);
        
        env.storage().persistent().set(&DataKey::VaultRebalanceHistory(vault_id.clone()), &history);
        env.storage().persistent().set(&DataKey::VaultAllocations(vault_id.clone()), &new_allocations);

        events::emit_rebalance(&env, vault_id, old_hash, new_hash);

        Ok(())
    }

    /// Pause or unpause deposits/withdrawals.
    pub fn set_status(
        env: Env,
        vault_id: BytesN<32>,
        deposit_status: VaultStatus,
        withdrawal_status: VaultStatus,
    ) -> Result<(), ContractError> {
        let mut config = Self::get_config(&env, &vault_id)?;
        config.manager.require_auth();

        config.deposit_status = deposit_status;
        config.withdrawal_status = withdrawal_status;

        env.storage().persistent().set(&DataKey::VaultConfig(vault_id.clone()), &config);
        events::emit_config_updated(&env, vault_id, config.manager);

        Ok(())
    }

    /// Claim accrued management fees.
    pub fn claim_fees(
        env: Env,
        vault_id: BytesN<32>,
    ) -> Result<Vec<(Address, i128)>, ContractError> {
        let config = Self::get_config(&env, &vault_id)?;
        config.manager.require_auth();

        let allocations = Self::get_allocations(&env, &vault_id)?;
        let mut claimed_amounts = Vec::new(&env);

        for alloc in allocations.iter() {
            let accrued = Self::get_accrued_fees(&env, &vault_id, &alloc.asset);
            if accrued > 0 {
                let token_client = token::Client::new(&env, &alloc.asset);
                token_client.transfer(&env.current_contract_address(), &config.manager, &accrued);
                
                env.storage().persistent().set(
                    &DataKey::VaultAccruedFees(vault_id.clone(), alloc.asset.clone()),
                    &0i128
                );
                claimed_amounts.push_back((alloc.asset, accrued));
            }
        }

        events::emit_fees_claimed(&env, vault_id, config.manager, claimed_amounts.clone());

        Ok(claimed_amounts)
    }

    // --- Internal Helpers ---

    fn validate_allocations(allocations: &Vec<AssetAllocation>) -> Result<(), ContractError> {
        let mut total_bps = 0u32;
        let mut assets = Vec::<Address>::new(allocations.env());

        for alloc in allocations.iter() {
            if assets.contains(&alloc.asset) {
                return Err(ContractError::DuplicateAsset);
            }
            assets.push_back(alloc.asset.clone());
            total_bps += alloc.target_bps;
        }

        if total_bps != math::BPS_LIMIT {
            return Err(ContractError::AllocationTotalMismatch);
        }

        Ok(())
    }

    fn calculate_total_vault_value(
        env: &Env,
        vault_id: &BytesN<32>,
        allocations: &Vec<AssetAllocation>,
    ) -> Result<i128, ContractError> {
        let mut total_value = 0i128;
        for alloc in allocations.iter() {
            let balance = Self::get_asset_balance(env, vault_id, &alloc.asset);
            total_value += balance;
        }
        Ok(total_value)
    }

    pub fn get_config(env: &Env, vault_id: &BytesN<32>) -> Result<VaultConfig, ContractError> {
        env.storage().persistent()
            .get(&DataKey::VaultConfig(vault_id.clone()))
            .ok_or(ContractError::VaultNotFound)
    }

    pub fn get_allocations(env: &Env, vault_id: &BytesN<32>) -> Result<Vec<AssetAllocation>, ContractError> {
        env.storage().persistent()
            .get(&DataKey::VaultAllocations(vault_id.clone()))
            .ok_or(ContractError::VaultNotFound)
    }

    pub fn get_total_shares(env: &Env, vault_id: &BytesN<32>) -> i128 {
        env.storage().persistent()
            .get(&DataKey::VaultTotalShares(vault_id.clone()))
            .unwrap_or(0)
    }

    pub fn get_user_shares(env: &Env, vault_id: &BytesN<32>, user: &Address) -> i128 {
        env.storage().persistent()
            .get(&DataKey::VaultUserShares(vault_id.clone(), user.clone()))
            .unwrap_or(0)
    }

    pub fn get_asset_balance(env: &Env, vault_id: &BytesN<32>, asset: &Address) -> i128 {
        env.storage().persistent()
            .get(&DataKey::VaultAssetBalance(vault_id.clone(), asset.clone()))
            .unwrap_or(0)
    }

    pub fn get_accrued_fees(env: &Env, vault_id: &BytesN<32>, asset: &Address) -> i128 {
        env.storage().persistent()
            .get(&DataKey::VaultAccruedFees(vault_id.clone(), asset.clone()))
            .unwrap_or(0)
    }
}
