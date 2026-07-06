# Contributing to Spectral Contracts

We welcome contributions to Spectral Verse! Whether you're fixing a bug, improving documentation, or suggesting new features, your help is appreciated.

## How to Contribute

### Finding Issues to Work On
- Browse the [issues](https://github.com/your-org/spectral-contracts/issues) page
- Look for labels like `good first issue`, `help wanted`, or `drips-wave` for beginner-friendly or Drips Wave-eligible tasks
- Comment on an issue to let others know you're working on it

### Drips Wave Contribution
This repository participates in [Drips Wave](https://docs.drips.network/wave/). Look for issues labeled `drips-wave` for eligible tasks.

## Local Setup

1.  **Clone the repository**: `git clone <repo-url>`
2.  **Install dependencies**: Ensure you have Rust and the Soroban CLI installed
3.  **Build the project**: Run `./scripts/build.sh`
4.  **Run tests**: Run `./scripts/test.sh`

## Coding Standards

- Use idiomatic Rust
- Follow the existing project structure
- Ensure all public functions are documented
- Maintain comprehensive test coverage for new features

## Pull Request Process

1.  Create a new branch from `main` (or the default branch) for your changes
2.  Make your changes and commit them with clear, descriptive messages
3.  Ensure all tests pass (`./scripts/test.sh`) and build passes (`./scripts/build.sh`)
4.  Submit a PR using the [PR template](.github/PULL_REQUEST_TEMPLATE.md), linking to the issue it addresses
5.  Documentation updates must accompany any functional changes
6.  A maintainer will review your PR and provide feedback

## Communication
- Use issues for bug reports and feature requests
- Use PR comments for discussion about specific changes
- Be respectful and follow our [Code of Conduct](CODE_OF_CONDUCT.md)

## Issue Guidelines

- Check if the issue already exists before opening a new one
- Provide clear steps to reproduce any bugs
- For feature requests, explain the use case and proposed implementation
- Use the appropriate issue template when creating a new issue
