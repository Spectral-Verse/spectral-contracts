# Spectra Contracts

Spectra is a suite of Soroban smart contracts on the Stellar network designed for creating and managing transparent, on-chain asset baskets. It allows strategy creators to define asset allocations, and users to participate in these baskets through a share-based vault system.

## Overview

Spectra Vaults provide a robust mechanism for:
- **Vault Creation**: Strategy creators can establish asset baskets with specific targets.
- **Asset Management**: Supported Stellar assets are managed according to configurable allocation rules.
- **Deposits & Withdrawals**: Users can deposit supported assets and receive shares, or redeem shares for their proportional share of the vault's assets.
- **Permissioned Rebalancing**: Authorized rebalance authorities can update target allocations.
- **Transparent Accounting**: Deterministic share accounting ensures all participants' positions are accurately tracked.

## User Roles

- **Vault Manager**: Responsible for creating vaults, updating metadata, managing fees, and pausing/unpausing operations.
- **Depositor**: Users who provide liquidity to the vault in exchange for shares.
- **Share Holder**: Users holding vault shares, entitled to proportional withdrawals.
- **Rebalance Authority**: A specialized role permitted to update the vault's asset allocation targets.

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
