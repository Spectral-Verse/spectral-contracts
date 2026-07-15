use crate::types::{AssetAllocation, RebalanceRecord, VaultConfig};
use soroban_sdk::{contracttype, Address, BytesN, Vec};

/// Keys used for persistent storage in the Spectra Vault contract.
///
/// Soroban uses a key-value store for state. These variants represent the
/// different types of data we persist for each vault.
#[contracttype]
pub enum DataKey {
    /// Static and dynamic settings for a vault (manager, fees, status).
    /// Maps: `BytesN<32> (VaultID) -> VaultConfig`
    VaultConfig(BytesN<32>),
    /// The list of target asset weights for a vault.
    /// Maps: `BytesN<32> (VaultID) -> Vec<AssetAllocation>`
    VaultAllocations(BytesN<32>),
    /// Total number of shares issued by a vault.
    /// Maps: `BytesN<32> (VaultID) -> i128`
    VaultTotalShares(BytesN<32>),
    /// Shares owned by a specific address in a specific vault.
    /// Maps: `(BytesN<32> (VaultID), Address) -> i128`
    VaultUserShares(BytesN<32>, Address),
    /// Current balance of an asset held by a specific vault.
    /// Maps: `(BytesN<32> (VaultID), Address) -> i128`
    VaultAssetBalance(BytesN<32>, Address),
    /// Unclaimed management fees accumulated for an asset in a vault.
    /// Maps: `(BytesN<32> (VaultID), Address) -> i128`
    VaultAccruedFees(BytesN<32>, Address),
    /// Historical log of rebalancing events for a vault.
    /// Maps: `BytesN<32> (VaultID) -> Vec<RebalanceRecord>`
    VaultRebalanceHistory(BytesN<32>),
}

// Storage helpers would go here if needed, but we'll implement them in the main lib or a helper trait.
