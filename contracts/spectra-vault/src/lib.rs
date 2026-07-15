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
use soroban_sdk::{contract, contractimpl, token, Address, BytesN, Env, String, Vec};

#[contract]
pub struct SpectralVault;

#[contractimpl]
impl SpectralVault {
    /// Creates a new asset basket vault with specified parameters.
    ///
    /// # Parameters
    /// * `vault_id` - A unique 32-byte identifier for the new vault.
    /// * `name` - Human-readable name for the strategy.
    /// * `metadata_hash` - Hash of off-chain strategy documentation.
    /// * `manager` - Address with administrative control over the vault.
    /// * `base_asset` - The primary asset used for valuation.
    /// * `rebalance_authority` - Address permitted to update asset allocations.
    /// * `management_fee_bps` - Management fee in basis points (max 500).
    /// * `allocations` - Initial asset composition and target weights.
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
        if env
            .storage()
            .persistent()
            .has(&DataKey::VaultConfig(vault_id.clone()))
        {
            return Err(ContractError::DuplicateVault);
        }

        if management_fee_bps > 500 {
            // Max 5% fee cap
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

        env.storage()
            .persistent()
            .set(&DataKey::VaultConfig(vault_id.clone()), &config);
        env.storage()
            .persistent()
            .set(&DataKey::VaultAllocations(vault_id.clone()), &allocations);
        env.storage()
            .persistent()
            .set(&DataKey::VaultTotalShares(vault_id.clone()), &0i128);

        events::emit_vault_created(&env, vault_id, manager);

        Ok(())
    }

    /// Deposits a supported asset into a vault and mints shares for the user.
    ///
    /// # Parameters
    /// * `vault_id` - ID of the vault to deposit into.
    /// * `user` - Address of the depositor.
    /// * `asset` - Address of the supported asset being deposited.
    /// * `amount` - Amount of the asset to deposit.
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
        let shares_to_mint =
            math::calculate_shares_to_mint(net_amount, total_shares, total_vault_value);

        // Transfer asset from user to contract
        let token_client = token::Client::new(&env, &asset);
        token_client.transfer(&user, &env.current_contract_address(), &amount);

        // Update accounting
        let new_total_shares = total_shares + shares_to_mint;
        env.storage().persistent().set(
            &DataKey::VaultTotalShares(vault_id.clone()),
            &new_total_shares,
        );

        let user_shares = Self::get_user_shares(&env, &vault_id, &user);
        env.storage().persistent().set(
            &DataKey::VaultUserShares(vault_id.clone(), user.clone()),
            &(user_shares + shares_to_mint),
        );

        let asset_balance = Self::get_asset_balance(&env, &vault_id, &asset);
        env.storage().persistent().set(
            &DataKey::VaultAssetBalance(vault_id.clone(), asset.clone()),
            &(asset_balance + net_amount),
        );

        if fee_amount > 0 {
            let accrued_fees = Self::get_accrued_fees(&env, &vault_id, &asset);
            env.storage().persistent().set(
                &DataKey::VaultAccruedFees(vault_id.clone(), asset.clone()),
                &(accrued_fees + fee_amount),
            );
        }

        events::emit_deposit(&env, vault_id, user, asset, net_amount, shares_to_mint);

        Ok(shares_to_mint)
    }

    /// Redeems vault shares for a proportional amount of all underlying assets.
    ///
    /// The withdrawal process burns the user's shares and returns a basket of assets
    /// proportional to their ownership of the total pool. This ensures that every
    /// shareholder is exposed to the same asset allocation and rebalancing risk.
    ///
    /// # Parameters
    /// * `vault_id` - ID of the vault to withdraw from.
    /// * `user` - Address of the shareholder requesting redemption.
    /// * `shares_to_burn` - Number of shares to redeem/burn.
    ///
    /// # Returns
    /// A vector of tuples containing (Asset Address, Withdrawn Amount) for each asset.
    ///
    /// # Errors
    /// * `ContractError::WithdrawalsPaused` - If the vault manager has disabled withdrawals.
    /// * `ContractError::InsufficientShares` - If the user doesn't own enough shares.
    /// * `ContractError::ZeroAmount` - If `shares_to_burn` is not positive.
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
            let amount_to_withdraw =
                math::calculate_proportional_amount(shares_to_burn, total_shares, asset_balance);

            if amount_to_withdraw > 0 {
                let token_client = token::Client::new(&env, &alloc.asset);
                token_client.transfer(&env.current_contract_address(), &user, &amount_to_withdraw);

                env.storage().persistent().set(
                    &DataKey::VaultAssetBalance(vault_id.clone(), alloc.asset.clone()),
                    &(asset_balance - amount_to_withdraw),
                );
                withdrawn_amounts.push_back((alloc.asset, amount_to_withdraw));
            }
        }

        // Update accounting
        let new_total_shares = total_shares - shares_to_burn;
        env.storage().persistent().set(
            &DataKey::VaultTotalShares(vault_id.clone()),
            &new_total_shares,
        );
        env.storage().persistent().set(
            &DataKey::VaultUserShares(vault_id.clone(), user.clone()),
            &(user_shares - shares_to_burn),
        );

        events::emit_withdrawal(
            &env,
            vault_id,
            user,
            shares_to_burn,
            withdrawn_amounts.clone(),
        );

        Ok(withdrawn_amounts)
    }

    /// Updates the target allocation weights for a vault's asset basket.
    ///
    /// This is a critical function used for rebalancing. The `rebalance_authority`
    /// defines new target weights for the assets in the basket. The actual movement
    /// of funds to match these weights happens through subsequent deposit/withdrawal
    /// or dedicated rebalancing trades (if implemented).
    ///
    /// # Parameters
    /// * `vault_id` - ID of the vault to update.
    /// * `new_allocations` - The new set of target weights and asset addresses.
    ///
    /// # Errors
    /// * `ContractError::Unauthorized` - If the caller is not the rebalance authority.
    /// * `ContractError::InvalidAllocation` - If weights don't sum to 10,000 BPS.
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

        let mut history: Vec<RebalanceRecord> = env
            .storage()
            .persistent()
            .get(&DataKey::VaultRebalanceHistory(vault_id.clone()))
            .unwrap_or(Vec::new(&env));
        history.push_back(record);

        env.storage()
            .persistent()
            .set(&DataKey::VaultRebalanceHistory(vault_id.clone()), &history);
        env.storage().persistent().set(
            &DataKey::VaultAllocations(vault_id.clone()),
            &new_allocations,
        );

        events::emit_rebalance(&env, vault_id, old_hash, new_hash);

        Ok(())
    }

    /// Updates the operational status of the vault (Active/Paused).
    ///
    /// This allows the manager to halt deposits or withdrawals in case of emergencies,
    /// protocol upgrades, or strategy changes.
    ///
    /// # Parameters
    /// * `vault_id` - ID of the vault to update.
    /// * `deposit_status` - New status for deposits (Active or Paused).
    /// * `withdrawal_status` - New status for withdrawals (Active or Paused).
    ///
    /// # Errors
    /// * `ContractError::Unauthorized` - If the caller is not the vault manager.
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

        env.storage()
            .persistent()
            .set(&DataKey::VaultConfig(vault_id.clone()), &config);
        events::emit_config_updated(&env, vault_id, config.manager);

        Ok(())
    }

    /// Allows the vault manager to claim all accrued management fees.
    ///
    /// Fees are collected during deposits and stored in the contract's balance.
    /// This function transfers those accrued amounts to the manager's address.
    ///
    /// # Parameters
    /// * `vault_id` - ID of the vault to claim fees from.
    ///
    /// # Returns
    /// A vector of tuples containing (Asset Address, Amount Claimed) for each asset.
    ///
    /// # Errors
    /// * `ContractError::Unauthorized` - If the caller is not the vault manager.
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
                    &0i128,
                );
                claimed_amounts.push_back((alloc.asset, accrued));
            }
        }

        events::emit_fees_claimed(&env, vault_id, config.manager, claimed_amounts.clone());

        Ok(claimed_amounts)
    }

    /// Validates that a set of allocations is correct.
    ///
    /// Checks:
    /// 1. No duplicate assets in the basket.
    /// 2. Total basis points sum exactly to 10,000 (100%).
    ///
    /// # Parameters
    /// * `allocations` - The list of asset allocations to validate.
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

    /// Internal helper to calculate the total current value of the vault.
    ///
    /// Sums up the balance of all assets currently held in the vault's basket.
    /// This value is used to determine the share price during deposits.
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

    /// Fetches the static configuration for a vault from persistent storage.
    ///
    /// # Parameters
    /// * `vault_id` - ID of the vault.
    ///
    /// # Returns
    /// The `VaultConfig` struct if found.
    pub fn get_config(env: &Env, vault_id: &BytesN<32>) -> Result<VaultConfig, ContractError> {
        env.storage()
            .persistent()
            .get(&DataKey::VaultConfig(vault_id.clone()))
            .ok_or(ContractError::VaultNotFound)
    }

    /// Fetches the current asset allocations for a vault from persistent storage.
    ///
    /// # Parameters
    /// * `vault_id` - ID of the vault.
    ///
    /// # Returns
    /// A vector of `AssetAllocation` structs.
    pub fn get_allocations(
        env: &Env,
        vault_id: &BytesN<32>,
    ) -> Result<Vec<AssetAllocation>, ContractError> {
        env.storage()
            .persistent()
            .get(&DataKey::VaultAllocations(vault_id.clone()))
            .ok_or(ContractError::VaultNotFound)
    }

    /// Returns the total number of shares issued for a specific vault.
    pub fn get_total_shares(env: &Env, vault_id: &BytesN<32>) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::VaultTotalShares(vault_id.clone()))
            .unwrap_or(0i128)
    }

    /// Returns the number of shares owned by a specific user in a vault.
    pub fn get_user_shares(env: &Env, vault_id: &BytesN<32>, user: &Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::VaultUserShares(vault_id.clone(), user.clone()))
            .unwrap_or(0i128)
    }

    /// Returns the current balance of a specific asset held by a vault.
    pub fn get_asset_balance(env: &Env, vault_id: &BytesN<32>, asset: &Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::VaultAssetBalance(vault_id.clone(), asset.clone()))
            .unwrap_or(0i128)
    }

    /// Returns the amount of accrued management fees for a specific asset in a vault.
    pub fn get_accrued_fees(env: &Env, vault_id: &BytesN<32>, asset: &Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::VaultAccruedFees(vault_id.clone(), asset.clone()))
            .unwrap_or(0i128)
    }
}
