# Testing Guide

Spectral Verse uses the standard Soroban Rust testing framework.

## Running Tests

To run all unit tests:
```bash
./scripts/test.sh
```
Or directly via cargo:
```bash
cargo test
```

## Main Test Scenarios

- **Vault Creation**: Verifies successful vault setup and error handling for invalid allocations and duplicate assets.
- **Deposits**: Tests asset transfers, share minting, and rejection of unsupported assets.
- **Withdrawals**: Tests full and partial withdrawals, ensuring proportional asset returns and share burning.
- **Access Control**: Ensures only authorized addresses can update configurations, rebalance, or claim fees.
- **Safety Controls**: Verifies that deposits and withdrawals can be paused and unpaused.
- **Fee Management**: Tests management fee accrual and claiming logic.

## Mocking Token Interactions

The tests use the `soroban-sdk` test utilities to mock Stellar Asset Contracts. This allows us to simulate token minting and transfers without needing a live network.

```rust
let asset_a = env.register_stellar_asset_contract(token_admin.clone());
let token_a = token::Client::new(&env, &asset_a);
token_a.mint(&user, &deposit_amount);
```

By using `env.mock_all_auths()`, we can focus on testing the contract logic while automatically satisfying the authorization requirements of the token contracts.
