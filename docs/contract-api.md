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

## Events

> **Note**: Event consumers should treat the topic order as stable API behaviour.

### `vault created`
- **Emitted when**: A new vault is successfully created.
- **Topics**: `["vault", "created", vault_id]`
- **Data**: `manager_address`

### `vault deposit`
- **Emitted when**: A user deposits an asset into a vault.
- **Topics**: `["vault", "deposit", vault_id]`
- **Data**: `(user_address, asset_address, net_amount, shares_minted)`

### `vault withdraw`
- **Emitted when**: A user withdraws assets from a vault.
- **Topics**: `["vault", "withdraw", vault_id]`
- **Data**: `(user_address, shares_burned, Vec<(asset_address, withdrawn_amount)>)`

### `vault rebalance`
- **Emitted when**: A vault's allocations are rebalanced.
- **Topics**: `["vault", "rebalance", vault_id]`
- **Data**: `(old_allocation_hash, new_allocation_hash)`

### `vault config`
- **Emitted when**: A vault's configuration or status is updated.
- **Topics**: `["vault", "config", vault_id]`
- **Data**: `manager_address`

### `vault fees`
- **Emitted when**: Accrued management fees are claimed by the manager.
- **Topics**: `["vault", "fees", vault_id]`
- **Data**: `(manager_address, Vec<(asset_address, amount_claimed)>)`
