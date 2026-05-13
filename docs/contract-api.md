# Contract API Reference

## Public Functions

### `create_vault`
Creates a new asset basket vault.
- **Parameters**:
  - `vault_id`: `BytesN<32>`
  - `name`: `String`
  - `metadata_hash`: `BytesN<32>`
  - `manager`: `Address`
  - `base_asset`: `Address`
  - `rebalance_authority`: `Address`
  - `management_fee_bps`: `u32`
  - `allocations`: `Vec<AssetAllocation>`
- **Errors**: `DuplicateVault`, `FeeTooHigh`, `AllocationTotalMismatch`, `DuplicateAsset`.
- **Events**: `vault created`.

### `deposit`
Deposits a supported asset into a vault and mints shares.
- **Parameters**:
  - `vault_id`: `BytesN<32>`
  - `user`: `Address`
  - `asset`: `Address`
  - `amount`: `i128`
- **Errors**: `VaultNotFound`, `DepositsPaused`, `InvalidAsset`, `ZeroAmount`.
- **Events**: `vault deposit`.

### `withdraw`
Redeems shares for proportional vault assets.
- **Parameters**:
  - `vault_id`: `BytesN<32>`
  - `user`: `Address`
  - `shares_to_burn`: `i128`
- **Errors**: `VaultNotFound`, `WithdrawalsPaused`, `InsufficientShares`, `ZeroAmount`.
- **Events**: `vault withdraw`.

### `update_allocations`
Updates the target allocations for a vault (Permissioned).
- **Parameters**:
  - `vault_id`: `BytesN<32>`
  - `new_allocations`: `Vec<AssetAllocation>`
- **Errors**: `VaultNotFound`, `Unauthorised`, `AllocationTotalMismatch`, `DuplicateAsset`.
- **Events**: `vault rebalance`.

### `set_status`
Pauses or unpauses vault operations (Permissioned).
- **Parameters**:
  - `vault_id`: `BytesN<32>`
  - `deposit_status`: `VaultStatus`
  - `withdrawal_status`: `VaultStatus`
- **Errors**: `VaultNotFound`, `Unauthorised`.
- **Events**: `vault config updated`.

### `claim_fees`
Claims accrued management fees for the vault manager (Permissioned).
- **Parameters**:
  - `vault_id`: `BytesN<32>`
- **Errors**: `VaultNotFound`, `Unauthorised`.
- **Events**: `vault fees claimed`.

## Read-only Functions

- `get_config(vault_id: BytesN<32>) -> VaultConfig`
- `get_allocations(vault_id: BytesN<32>) -> Vec<AssetAllocation>`
- `get_total_shares(vault_id: BytesN<32>) -> i128`
- `get_user_shares(vault_id: BytesN<32>, user: Address) -> i128`
- `get_asset_balance(vault_id: BytesN<32>, asset: Address) -> i128`
- `get_accrued_fees(vault_id: BytesN<32>, asset: Address) -> i128`
