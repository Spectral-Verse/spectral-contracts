/// Total basis points (100.00%)
pub const BPS_LIMIT: u32 = 10_000;

/// Calculates the number of shares to mint for a given deposit value.
///
/// # Arguments
/// * `deposit_value` - The net value of the asset being deposited.
/// * `total_shares` - The current total number of shares in the vault.
/// * `total_vault_value` - The current total value of all assets in the vault.
pub fn calculate_shares_to_mint(
    deposit_value: i128,
    total_shares: i128,
    total_vault_value: i128,
) -> i128 {
    if total_shares == 0 || total_vault_value == 0 {
        return deposit_value;
    }
    // shares = (deposit_value * total_shares) / total_vault_value
    (deposit_value * total_shares) / total_vault_value
}

/// Calculates the proportional amount of an asset to withdraw based on shares burned.
///
/// # Arguments
/// * `user_shares` - The number of shares being redeemed/burned.
/// * `total_shares` - The current total number of shares in the vault.
/// * `asset_balance` - The current balance of the specific asset in the vault.
pub fn calculate_proportional_amount(
    user_shares: i128,
    total_shares: i128,
    asset_balance: i128,
) -> i128 {
    if total_shares == 0 {
        return 0;
    }
    // amount = (user_shares * asset_balance) / total_shares
    (user_shares * asset_balance) / total_shares
}

/// Calculates the fee amount based on basis points.
///
/// # Arguments
/// * `amount` - The total amount to calculate the fee from.
/// * `fee_bps` - The fee in basis points (e.g., 100 = 1%).
pub fn calculate_fee(amount: i128, fee_bps: u32) -> i128 {
    (amount * (fee_bps as i128)) / (BPS_LIMIT as i128)
}
