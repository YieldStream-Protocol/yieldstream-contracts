# Contributing to YieldStream Contracts

We welcome open-source contributions to scale Stellar's DeFi TVL!

## Development Workflow
1. **Fork & Clone:** Fork the repo and clone it locally.
2. **Branching:** Use `feat/`, `fix/`, or `chore/` prefixes.
3. **DeFi Testing Standard:** You MUST write unit tests in `src/test.rs` for any logic changes. Because this is DeFi software, tests must include zero-value edge cases, high-slippage scenarios, and precision-loss checks. PRs without adequate test coverage will be rejected.
4. **Formatting:** Ensure `make fmt` passes before opening a PR.

Search the codebase for `// TODO:` comments to find areas that need immediate help.