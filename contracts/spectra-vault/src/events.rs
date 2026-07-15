use crate::types::AssetAllocation;
use soroban_sdk::{symbol_short, Address, BytesN, Env, Symbol, Vec};

/// Emits an event when a new vault is successfully created.
///
/// Topics: `["vault", "created", vault_id]`
/// Data: `manager_address`
pub fn emit_vault_created(env: &Env, vault_id: BytesN<32>, manager: Address) {
    let topics = (symbol_short!("vault"), symbol_short!("created"), vault_id);
    env.events().publish(topics, manager);
}

/// Emits an event when a user deposits an asset into a vault.
///
/// Topics: `["vault", "deposit", vault_id]`
/// Data: `(user_address, asset_address, net_amount, shares_minted)`
pub fn emit_deposit(
    env: &Env,
    vault_id: BytesN<32>,
    user: Address,
    asset: Address,
    amount: i128,
    shares: i128,
) {
    let topics = (symbol_short!("vault"), symbol_short!("deposit"), vault_id);
    env.events().publish(topics, (user, asset, amount, shares));
}

/// Emits an event when a user withdraws assets from a vault.
///
/// Topics: `["vault", "withdraw", vault_id]`
/// Data: `(user_address, shares_burned, Vec<(asset_address, withdrawn_amount)>)`
pub fn emit_withdrawal(
    env: &Env,
    vault_id: BytesN<32>,
    user: Address,
    shares: i128,
    amounts: Vec<(Address, i128)>,
) {
    let topics = (symbol_short!("vault"), symbol_short!("withdraw"), vault_id);
    env.events().publish(topics, (user, shares, amounts));
}

/// Emits an event when a vault's allocations are rebalanced.
///
/// Topics: `["vault", "rebalance", vault_id]`
/// Data: `(old_allocation_hash, new_allocation_hash)`
pub fn emit_rebalance(env: &Env, vault_id: BytesN<32>, old_hash: BytesN<32>, new_hash: BytesN<32>) {
    let topics = (symbol_short!("vault"), symbol_short!("rebalance"), vault_id);
    env.events().publish(topics, (old_hash, new_hash));
}

/// Emits an event when a vault's configuration or status is updated.
///
/// Topics: `["vault", "config", vault_id]`
/// Data: `manager_address`
pub fn emit_config_updated(env: &Env, vault_id: BytesN<32>, manager: Address) {
    let topics = (symbol_short!("vault"), symbol_short!("config"), vault_id);
    env.events().publish(topics, manager);
}

/// Emits an event when accrued management fees are claimed by the manager.
///
/// Topics: `["vault", "fees", vault_id]`
/// Data: `(manager_address, Vec<(asset_address, amount_claimed)>)`
pub fn emit_fees_claimed(
    env: &Env,
    vault_id: BytesN<32>,
    manager: Address,
    amounts: Vec<(Address, i128)>,
) {
    let topics = (symbol_short!("vault"), symbol_short!("fees"), vault_id);
    env.events().publish(topics, (manager, amounts));
}
