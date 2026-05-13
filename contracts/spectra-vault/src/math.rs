pub const BPS_LIMIT: u32 = 10_000;

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

pub fn calculate_fee(amount: i128, fee_bps: u32) -> i128 {
    (amount * (fee_bps as i128)) / (BPS_LIMIT as i128)
}
