# Spectra Contracts

Spectra is a production-grade suite of Soroban smart contracts on the Stellar network designed for creating and managing transparent, on-chain asset baskets. It enables strategy creators to define sophisticated asset allocations and allows participants to engage with these baskets through a robust, share-based vault architecture.

## Core Capabilities

- **Vault Lifecycle Management**: Establish asset baskets with precise configuration, management roles, and operational status controls.
- **Dynamic Asset Allocation**: Support for multi-asset baskets with target weights defined in basis points.
- **Precision Accounting**: Deterministic share-based accounting ensures accurate tracking of user positions and proportional asset ownership.
- **Permissioned Rebalancing**: Secure mechanisms for authorized authorities to update strategy targets.
- **Safety Controls**: Built-in pausing mechanisms for deposits and withdrawals to protect participant funds during maintenance or emergencies.

## System Roles

- **Vault Manager**: Oversees vault initialization, metadata updates, fee management, and operational status.
- **Rebalance Authority**: Specialized role responsible for adjusting asset allocation targets based on strategy requirements.
- **Depositor/Shareholder**: Participants who provide liquidity to the vault in exchange for minted shares representing their claim.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup#install-the-soroban-cli)
- [Stellar Network (Testnet/Futurenet)](https://soroban.stellar.org/docs/getting-started/deploy-to-testnet)

### Build

To build the contracts, run:
```bash
./scripts/build.sh
```

### Test

To run the unit tests:
```bash
./scripts/test.sh
```

### Deployment

Deploy the contract to Stellar Testnet:
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/spectra_vault.wasm \
  --source-account <YOUR_ACCOUNT> \
  --network testnet
```

## Contract API

For a detailed overview of the contract functions, see [contract-api.md](docs/contract-api.md).

## Security

Spectra is an open-source project. While we prioritize security and follow best practices, the contracts have not undergone a formal third-party audit. 

**Warning**: Users should perform their own review before using these contracts with significant funds. Do not rely on these contracts for high-value assets without independent verification.

For more details, see [SECURITY.md](SECURITY.md) and [security-model.md](docs/security-model.md).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
