#![cfg(test)]

use crate::{types::{AssetAllocation, VaultStatus}, SpectralVault, SpectralVaultClient};
use soroban_sdk::{testutils::{Address as _, Ledger}, token, Address, BytesN, Env, String, Vec};

fn setup_test_env(env: &Env) -> (SpectralVaultClient, Address, Address, Address, Address, Address) {
    let contract_id = env.register_contract(None, SpectralVault);
    let client = SpectralVaultClient::new(env, &contract_id);

    let manager = Address::generate(env);
    let user = Address::generate(env);
    let rebalance_auth = Address::generate(env);
    
    let token_admin = Address::generate(env);
    let asset_a = env.register_stellar_asset_contract(token_admin.clone());
    let asset_b = env.register_stellar_asset_contract(token_admin.clone());

    (client, manager, user, rebalance_auth, asset_a, asset_b)
}

#[test]
fn test_create_vault() {
    let env = Env::default();
    let (client, manager, _, rebalance_auth, asset_a, asset_b) = setup_test_env(&env);

    let vault_id = BytesN::from_array(&env, &[0u8; 32]);
    let allocations = Vec::from_array(&env, [
        AssetAllocation { asset: asset_a.clone(), target_bps: 5000 },
        AssetAllocation { asset: asset_b.clone(), target_bps: 5000 },
    ]);

    client.create_vault(
        &vault_id,
        &String::from_str(&env, "Test Vault"),
        &BytesN::from_array(&env, &[0u8; 32]),
        &manager,
        &asset_a,
        &rebalance_auth,
        &100, // 1% fee
        &allocations,
    );

    let config = client.get_config(&vault_id);
    assert_eq!(config.name, String::from_str(&env, "Test Vault"));
    assert_eq!(config.manager, manager);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #7)")]
fn test_create_vault_invalid_allocation() {
    let env = Env::default();
    let (client, manager, _, rebalance_auth, asset_a, _) = setup_test_env(&env);

    let vault_id = BytesN::from_array(&env, &[0u8; 32]);
    let allocations = Vec::from_array(&env, [
        AssetAllocation { asset: asset_a.clone(), target_bps: 9000 },
    ]);

    client.create_vault(
        &vault_id,
        &String::from_str(&env, "Invalid Vault"),
        &BytesN::from_array(&env, &[0u8; 32]),
        &manager,
        &asset_a,
        &rebalance_auth,
        &100,
        &allocations,
    );
}

#[test]
fn test_deposit() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, manager, user, rebalance_auth, asset_a, asset_b) = setup_test_env(&env);

    let vault_id = BytesN::from_array(&env, &[0u8; 32]);
    let allocations = Vec::from_array(&env, [
        AssetAllocation { asset: asset_a.clone(), target_bps: 5000 },
        AssetAllocation { asset: asset_b.clone(), target_bps: 5000 },
    ]);

    client.create_vault(
        &vault_id,
        &String::from_str(&env, "Test Vault"),
        &BytesN::from_array(&env, &[0u8; 32]),
        &manager,
        &asset_a,
        &rebalance_auth,
        &0, // 0% fee
        &allocations,
    );

    let deposit_amount = 1000i128;
    let token_a = token::Client::new(&env, &asset_a);
    
    // Use token_admin logic via mock_all_auths or manual minting
    // Since we're using register_stellar_asset_contract, we can mint
    token_a.mint(&user, &deposit_amount);

    let shares = client.deposit(&vault_id, &user, &asset_a, &deposit_amount);
    assert_eq!(shares, deposit_amount); // First deposit: shares = deposit_amount
    assert_eq!(client.get_user_shares(&vault_id, &user), shares);
    assert_eq!(client.get_asset_balance(&vault_id, &asset_a), deposit_amount);
    assert_eq!(client.get_accrued_fees(&vault_id, &asset_a), 0);
    assert_eq!(token_a.balance(&client.address), deposit_amount);
}

#[test]
fn test_deposit_with_fee() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, manager, user, rebalance_auth, asset_a, asset_b) = setup_test_env(&env);

    let vault_id = BytesN::from_array(&env, &[1u8; 32]);
    let allocations = Vec::from_array(&env, [
        AssetAllocation { asset: asset_a.clone(), target_bps: 5000 },
        AssetAllocation { asset: asset_b.clone(), target_bps: 5000 },
    ]);

    client.create_vault(
        &vault_id,
        &String::from_str(&env, "Fee Vault"),
        &BytesN::from_array(&env, &[0u8; 32]),
        &manager,
        &asset_a,
        &rebalance_auth,
        &100, // 1% fee
        &allocations,
    );

    let deposit_amount = 1000i128;
    let fee_amount = 10i128;
    let net_amount = deposit_amount - fee_amount;
    let token_a = token::Client::new(&env, &asset_a);
    
    token_a.mint(&user, &deposit_amount);

    let shares = client.deposit(&vault_id, &user, &asset_a, &deposit_amount);
    
    assert_eq!(shares, net_amount);
    assert_eq!(client.get_user_shares(&vault_id, &user), net_amount);
    assert_eq!(client.get_asset_balance(&vault_id, &asset_a), net_amount);
    assert_eq!(client.get_accrued_fees(&vault_id, &asset_a), fee_amount);
    assert_eq!(token_a.balance(&client.address), deposit_amount);
}

#[test]
fn test_withdraw_full() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, manager, user, rebalance_auth, asset_a, asset_b) = setup_test_env(&env);

    let vault_id = BytesN::from_array(&env, &[0u8; 32]);
    let allocations = Vec::from_array(&env, [
        AssetAllocation { asset: asset_a.clone(), target_bps: 5000 },
        AssetAllocation { asset: asset_b.clone(), target_bps: 5000 },
    ]);

    client.create_vault(&vault_id, &String::from_str(&env, "Vault"), &BytesN::from_array(&env, &[0u8; 32]), &manager, &asset_a, &rebalance_auth, &0, &allocations);

    let deposit_amount = 1000i128;
    token::Client::new(&env, &asset_a).mint(&user, &deposit_amount);
    client.deposit(&vault_id, &user, &asset_a, &deposit_amount);

    let shares_to_burn = client.get_user_shares(&vault_id, &user);
    client.withdraw(&vault_id, &user, &shares_to_burn);

    assert_eq!(client.get_user_shares(&vault_id, &user), 0);
    assert_eq!(client.get_total_shares(&vault_id), 0);
    assert_eq!(token::Client::new(&env, &asset_a).balance(&user), deposit_amount);
}

#[test]
fn test_paused_deposit() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, manager, user, rebalance_auth, asset_a, _) = setup_test_env(&env);

    let vault_id = BytesN::from_array(&env, &[0u8; 32]);
    let allocations = Vec::from_array(&env, [AssetAllocation { asset: asset_a.clone(), target_bps: 10000 }]);
    client.create_vault(&vault_id, &String::from_str(&env, "Vault"), &BytesN::from_array(&env, &[0u8; 32]), &manager, &asset_a, &rebalance_auth, &0, &allocations);

    client.set_status(&vault_id, &VaultStatus::Paused, &VaultStatus::Active);

    token::Client::new(&env, &asset_a).mint(&user, &1000);
    
    let result = client.try_deposit(&vault_id, &user, &asset_a, &1000);
    assert!(result.is_err());
}
