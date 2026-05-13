# Security Model

## Trust Assumptions

- **Manager**: Users trust the Vault Manager to manage metadata, fees, and operational status responsibly.
- **Rebalance Authority**: Users trust the Rebalance Authority to set appropriate allocation targets.
- **Stellar Network**: The security of the contracts relies on the underlying security of the Stellar network and the Soroban runtime.

## Role Permissions

### Vault Manager
- Update vault metadata.
- Set deposit and withdrawal status (pause/unpause).
- Claim accrued management fees.

### Rebalance Authority
- Update asset allocation targets.

## Limitations

- **No On-chain Price Oracles**: Currently, the vault calculates value based on asset balances directly. It does not account for asset prices in a common base currency (e.g., XLM or USD) unless the assets are inherently stable relative to each other.
- **Manual Rebalancing Execution**: The contract tracks target allocations but does not execute swaps. Actual rebalancing must be performed externally and then recorded on-chain.

## Risks

- **Arithmetic Errors**: While using safe math patterns, extreme values or very small increments could lead to rounding issues.
- **External Token Risk**: If an underlying asset contract is compromised or has malicious logic, it could affect the vault's solvency.
- **Governance Risk**: Broad powers held by the Manager or Rebalance Authority could be misused if not properly governed (e.g., via a multisig or DAO).

## Recommendations

- **Independent Review**: These contracts should undergo a thorough independent security review before being used for significant fund management.
- **Gradual Rollout**: Start with small limits and increase them as confidence in the system grows.
