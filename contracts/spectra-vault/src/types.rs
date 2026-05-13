use soroban_sdk::{contracttype, Address, BytesN, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VaultStatus {
    Active,
    Paused,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetAllocation {
    pub asset: Address,
    pub target_bps: u32, // Basis points (10,000 = 100%)
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultConfig {
    pub name: String,
    pub metadata_hash: BytesN<32>,
    pub manager: Address,
    pub base_asset: Address,
    pub rebalance_authority: Address,
    pub management_fee_bps: u32,
    pub deposit_status: VaultStatus,
    pub withdrawal_status: VaultStatus,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RebalanceRecord {
    pub timestamp: u64,
    pub old_allocation_hash: BytesN<32>,
    pub new_allocation_hash: BytesN<32>,
}
