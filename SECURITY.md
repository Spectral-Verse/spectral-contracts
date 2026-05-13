# Security Policy

## Responsible Disclosure

If you discover a security vulnerability within this project, please send an e-mail to the maintainers. All security vulnerabilities will be promptly addressed.

## Supported Scope

The following contracts are in scope for security reports:
- `spectra-vault`

## Known Limitations

- **No Built-in Price Oracles**: The current version relies on fixed allocation targets and does not integrate directly with on-chain price oracles for valuation.
- **Manager Permissions**: The Vault Manager has significant control over metadata and fee parameters.
- **Rebalance Authority**: The Rebalance Authority can change allocation targets, which affects the underlying composition of the vault.

## Risk Notes

- **Accounting Risk**: While deterministic math is used, rounding errors in share calculations can occur, especially with very small amounts.
- **Fee Risk**: Management fees are accrued based on the defined basis points. High fees can significantly impact depositor returns.
- **External Integration Risk**: The contracts interact with external Stellar assets (SAC). Any issues with those external tokens may affect the vault.

**DO NOT rely on these contracts for high-value funds without an independent security review.**
