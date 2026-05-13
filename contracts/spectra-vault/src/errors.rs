use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ContractError {
    Unauthorised = 1,
    VaultNotFound = 2,
    InvalidVaultState = 3,
    InvalidAsset = 4,
    DuplicateAsset = 5,
    InvalidAllocation = 6,
    AllocationTotalMismatch = 7,
    DepositsPaused = 8,
    WithdrawalsPaused = 9,
    ZeroAmount = 10,
    InsufficientShares = 11,
    InsufficientBalance = 12,
    FeeTooHigh = 13,
    DuplicateVault = 14,
    InvalidRebalance = 15,
    AssetTransferFailure = 16,
    MathOverflow = 17,
}
