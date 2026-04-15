# 🌊 YieldStream Contracts

Core DeFi smart contract infrastructure for the YieldStream Protocol. Built on the Stellar network using the Soroban SDK.

## Overview
YieldStream is an automated Yield Aggregator. Users deposit base assets into a Vault, receive share tokens (ysTokens), and the protocol automatically deploys that liquidity across whitelisted DeFi strategies to maximize APY.

## Prerequisites
* [Rust](https://rustup.rs/) (>= 1.71)
* [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup)

## Setup & Build
Compile the smart contracts to WASM:
```bash
make build
```

## Testing
Run the comprehensive unit tests locally:
```bash
make test
```
## Contributing
Please see CONTRIBUTING.md for guidelines on how to submit pull requests, branch naming conventions, and mathematical testing requirements.