use soroban_sdk::{symbol_short, Address, BytesN, Env, Symbol, Vec};
use crate::types::AssetAllocation;

pub fn emit_vault_created(env: &Env, vault_id: BytesN<32>, manager: Address) {
    let topics = (symbol_short!("vault"), symbol_short!("created"), vault_id);
    env.events().publish(topics, manager);
}

pub fn emit_deposit(env: &Env, vault_id: BytesN<32>, user: Address, asset: Address, amount: i128, shares: i128) {
    let topics = (symbol_short!("vault"), symbol_short!("deposit"), vault_id);
    env.events().publish(topics, (user, asset, amount, shares));
}

pub fn emit_withdrawal(env: &Env, vault_id: BytesN<32>, user: Address, shares: i128, amounts: Vec<(Address, i128)>) {
    let topics = (symbol_short!("vault"), symbol_short!("withdraw"), vault_id);
    env.events().publish(topics, (user, shares, amounts));
}

pub fn emit_rebalance(env: &Env, vault_id: BytesN<32>, old_hash: BytesN<32>, new_hash: BytesN<32>) {
    let topics = (symbol_short!("vault"), symbol_short!("rebalance"), vault_id);
    env.events().publish(topics, (old_hash, new_hash));
}

pub fn emit_config_updated(env: &Env, vault_id: BytesN<32>, manager: Address) {
    let topics = (symbol_short!("vault"), symbol_short!("config"), vault_id);
    env.events().publish(topics, manager);
}

pub fn emit_fees_claimed(env: &Env, vault_id: BytesN<32>, manager: Address, amounts: Vec<(Address, i128)>) {
    let topics = (symbol_short!("vault"), symbol_short!("fees"), vault_id);
    env.events().publish(topics, (manager, amounts));
}
