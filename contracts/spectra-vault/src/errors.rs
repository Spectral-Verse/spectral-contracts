use soroban_sdk::contracterror;

/// Custom error types for the Spectra Vault contract.
/// 
/// These errors are returned by contract functions when business logic
/// constraints are violated.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ContractError {
    /// Caller does not have the required permissions for the operation.
    /// Triggered when `require_auth()` fails for a specific role (Manager, etc).
    Unauthorised = 1,
    /// The requested vault ID was not found in persistent storage.
    VaultNotFound = 2,
    /// The vault is in an invalid state (e.g., trying to modify a closed vault).
    InvalidVaultState = 3,
    /// The provided asset address is not part of the vault's approved basket.
    InvalidAsset = 4,
    /// The same asset address was provided multiple times in an allocation list.
    DuplicateAsset = 5,
    /// A single asset's allocation weight is invalid (e.g., negative or too high).
    InvalidAllocation = 6,
    /// The sum of all asset target weights does not equal 10,000 basis points.
    AllocationTotalMismatch = 7,
    /// The vault manager has temporarily disabled new deposits.
    DepositsPaused = 8,
    /// The vault manager has temporarily disabled share redemptions.
    WithdrawalsPaused = 9,
    /// The amount provided for deposit or withdrawal must be strictly positive.
    ZeroAmount = 10,
    /// The user is attempting to withdraw more shares than they currently own.
    InsufficientShares = 11,
    /// The vault contract lacks sufficient balance of an underlying asset.
    InsufficientBalance = 12,
    /// The proposed management fee exceeds the system-wide maximum cap (5%).
    FeeTooHigh = 13,
    /// Attempting to create a new vault with an ID that is already registered.
    DuplicateVault = 14,
    /// The rebalance request contains invalid logic or inconsistent data.
    InvalidRebalance = 15,
    /// The underlying Stellar Asset Contract (SAC) transfer operation failed.
    AssetTransferFailure = 16,
    /// An internal arithmetic calculation resulted in an integer overflow.
    MathOverflow = 17,
}
