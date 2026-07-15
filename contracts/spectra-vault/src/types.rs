use soroban_sdk::{contracttype, Address, BytesN, String, Vec};

/// Operational status of a vault.
///
/// Used to control whether specific operations like deposits or withdrawals
/// are currently allowed.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VaultStatus {
    /// Vault is fully operational and accepting transactions.
    Active,
    /// Vault operations are suspended, typically for maintenance or emergencies.
    Paused,
}

/// Defines the target weight of a single asset within the vault's basket.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetAllocation {
    /// Address of the supported Stellar asset (SAC-compatible).
    pub asset: Address,
    /// Target allocation weight in basis points.
    /// 1 basis point (bps) = 0.01%, so 10,000 bps = 100.00%.
    pub target_bps: u32,
}

/// Static and dynamic configuration parameters for a Spectral Verse vault.
///
/// This struct is stored in persistent storage and defines the core
/// behavior and permissions of the vault.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultConfig {
    /// Human-readable name of the vault (e.g., "Blue Chip Index").
    pub name: String,
    /// cryptographic hash (SHA-256) of off-chain strategy metadata,
    /// providing a link to detailed documentation or strategy logic.
    pub metadata_hash: BytesN<32>,
    /// Address with administrative permissions (can change status, claim fees).
    pub manager: Address,
    /// The primary asset used for valuation and share price calculation.
    pub base_asset: Address,
    /// Address permitted to update target allocations (rebalancing).
    pub rebalance_authority: Address,
    /// Management fee charged on deposits, expressed in basis points.
    /// Max value is capped by the contract logic (e.g., 500 bps).
    pub management_fee_bps: u32,
    /// Current operational status for new deposits.
    pub deposit_status: VaultStatus,
    /// Current operational status for share redemptions.
    pub withdrawal_status: VaultStatus,
}

/// Represents a historical record of a rebalancing event.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RebalanceRecord {
    /// Ledger timestamp when the rebalance was committed.
    pub timestamp: u64,
    /// Hash of the asset allocation basket prior to this update.
    pub old_allocation_hash: BytesN<32>,
    /// Hash of the asset allocation basket after this update.
    pub new_allocation_hash: BytesN<32>,
}
