# Roadmap

Spectra is an evolving project with several planned enhancements.

## Future Improvements

- **Oracle-Based Valuation**: Integrate with on-chain price oracles (e.g., Band Protocol, Pyth) to support more complex valuation and rebalancing logic based on market prices.
- **Automated Rebalance Execution**: Develop integrations with Soroban-based DEXs (e.g., Soroswap) to allow the contract to automatically execute swaps to meet target allocations.
- **Multi-Manager Permissions**: Support multisig or DAO-based governance for vault management and rebalancing.
- **Strategy Performance Indexing**: Create off-chain indexers to track and display the historical performance of different vault strategies.
- **Enhanced Dashboard Integration**: Build a user-friendly frontend for strategy creators to manage vaults and for users to track their positions.
- **Event Indexing Examples**: Provide reference implementations for indexing Spectra events using tools like Mercury or custom subgraphs.
- **Property-Based Testing**: Implement property tests (e.g., using `proptest`) to ensure the robustness of share accounting under a wide range of scenarios.
- **Formal Specification**: Develop a formal specification for the vault lifecycle and state transitions to further improve security and correctness.
- **Stellar Asset Contract (SAC) Flows**: Optimize integration with the latest Stellar Asset Contract features and standards.
