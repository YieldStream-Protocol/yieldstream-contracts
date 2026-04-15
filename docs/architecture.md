# YieldStream Architecture

## Core Components
YieldStream is designed with a strict separation of concerns to maximize security and composability:

### 1. The Vault (User Facing)
The `vault.rs` module acts as the treasury. It holds the underlying user deposits (e.g., USDC) and issues `ysTokens` in return. The Vault is responsible for all accounting, share pricing, and processing user deposits/withdrawals.

### 2. The Strategies (Capital Deployment)
The `strategy.rs` module manages how the capital generates yield. Rather than locking users directly into external protocols, the Vault routes funds to whitelisted Strategies. This allows the Vault to dynamically rebalance funds between different lending protocols to chase the highest APY without users needing to manually migrate their tokens.

## Security Model
* **Non-Custodial:** YieldStream never takes ownership of user funds. Users can burn their `ysTokens` at any time to reclaim their share of the vault.
* **Circuit Breaker:** The `emergency_pause` function allows the protocol admin to instantly halt all deposits and withdrawals. This is a critical safety mechanism designed to freeze the vault state if an external integrated DeFi protocol (Strategy) is exploited, preventing malicious actors from draining the Vault's underlying liquidity.