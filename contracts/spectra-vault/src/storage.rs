use soroban_sdk::{contracttype, Address, BytesN, Vec};
use crate::types::{VaultConfig, AssetAllocation, RebalanceRecord};

#[contracttype]
pub enum DataKey {
    VaultConfig(BytesN<32>),
    VaultAllocations(BytesN<32>),
    VaultTotalShares(BytesN<32>),
    VaultUserShares(BytesN<32>, Address),
    VaultAssetBalance(BytesN<32>, Address),
    VaultAccruedFees(BytesN<32>, Address),
    VaultRebalanceHistory(BytesN<32>),
}

// Storage helpers would go here if needed, but we'll implement them in the main lib or a helper trait.
