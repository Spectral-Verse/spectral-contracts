# Architecture

Spectra is designed as a modular vault system for Stellar assets.

## System Components

- **Spectra Vault Contract**: The core logic handling accounting, deposits, withdrawals, and permissioned rebalancing.
- **Stellar Asset Contracts (SAC)**: External token contracts that the vault interacts with for asset transfers.

## Vault Lifecycle

1.  **Creation**: A manager initializes a vault with a unique ID, name, metadata hash, and initial allocation targets.
2.  **Active**: The vault accepts deposits and allows withdrawals.
3.  **Rebalancing**: The rebalance authority updates target allocations.
4.  **Paused**: The manager can pause deposits or withdrawals in case of emergency or maintenance.

## Share Accounting

Spectra uses a deterministic share-based accounting system:
- **First Deposit**: Shares minted = amount deposited.
- **Subsequent Deposits**: `shares = (deposit_value * total_shares) / total_vault_value`.
- **Withdrawals**: `asset_amount = (shares_to_burn * asset_balance) / total_shares`.

## Asset Basket Configuration

Each vault maintains a list of supported assets and their target allocation weights (in basis points). The total weight must always equal 10,000 bps (100%).

## Rebalance Authority

The rebalance authority is a specialized role that can update the `VaultAllocations`. This role is separate from the Vault Manager to allow for specialized strategy management.

## Metadata Hashes

The `metadata_hash` field allows strategy creators to link detailed off-chain strategy descriptions (e.g., stored on IPFS) to the on-chain vault record. This ensures transparency while keeping the on-chain footprint minimal.

## Event Design

Spectra emits structured events for all major state changes, including:
- Vault creation
- Deposits/Withdrawals
- Configuration updates
- Rebalancing actions
- Fee claims

These events are designed to be easily indexed by external tools for building dashboards and analytics.
