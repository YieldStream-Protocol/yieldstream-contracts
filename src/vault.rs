use crate::{DataKey, YieldStreamVault, YieldStreamVaultClient};
use soroban_sdk::{contractimpl, token, Address, Env};

#[contractimpl]
impl YieldStreamVault {
    /// Deposits underlying assets into the vault and mints ysTokens to the user.
    pub fn deposit(env: Env, user: Address, amount: i128) -> i128 {
        user.require_auth();
        assert_vault_active(&env);
        assert!(amount > 0, "Deposit amount must be strictly positive");

        // TODO: Add slippage protection parameters to the deposit function

        let underlying: Address = env.storage().instance().get(&DataKey::UnderlyingToken).unwrap();
        let share_token: Address = env.storage().instance().get(&DataKey::ShareToken).unwrap();
        let mut total_shares: i128 = env.storage().instance().get(&DataKey::TotalShares).unwrap();

        let underlying_client = token::Client::new(&env, &underlying);
        let share_client = token::StellarAssetClient::new(&env, &share_token);

        let vault_balance = underlying_client.balance(&env.current_contract_address());

        // Calculate shares to mint
        let shares_to_mint = if total_shares == 0 || vault_balance == 0 {
            amount // 1:1 exchange rate on initial deposit
        } else {
            (amount * total_shares) / vault_balance
        };

        // Transfer underlying tokens from user to vault
        underlying_client.transfer(&user, &env.current_contract_address(), &amount);

        // Mint share tokens to the user
        share_client.mint(&user, &shares_to_mint);

        // Update total shares state
        total_shares += shares_to_mint;
        env.storage().instance().set(&DataKey::TotalShares, &total_shares);

        shares_to_mint
    }

    /// Burns ysTokens to withdraw principal plus accrued yield.
    pub fn withdraw(env: Env, user: Address, shares: i128) -> i128 {
        user.require_auth();
        assert_vault_active(&env);
        assert!(shares > 0, "Withdrawal shares must be strictly positive");

        let underlying: Address = env.storage().instance().get(&DataKey::UnderlyingToken).unwrap();
        let share_token: Address = env.storage().instance().get(&DataKey::ShareToken).unwrap();
        let mut total_shares: i128 = env.storage().instance().get(&DataKey::TotalShares).unwrap();

        let underlying_client = token::Client::new(&env, &underlying);
        let share_client = token::Client::new(&env, &share_token);

        let vault_balance = underlying_client.balance(&env.current_contract_address());

        // Calculate underlying assets to return
        let underlying_out = (shares * vault_balance) / total_shares;

        // Burn user's share tokens
        share_client.burn(&user, &shares);

        // Transfer underlying assets back to user
        underlying_client.transfer(&env.current_contract_address(), &user, &underlying_out);

        // Update total shares state
        total_shares -= shares;
        env.storage().instance().set(&DataKey::TotalShares, &total_shares);

        underlying_out
    }

    /// Circuit breaker to halt all deposits and withdrawals during an emergency.
    pub fn emergency_pause(env: Env, admin: Address, pause: bool) {
        admin.require_auth();
        let current_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        assert_eq!(admin, current_admin, "Unauthorized");

        // TODO: Implement a time-lock for admin upgrades
        env.storage().instance().set(&DataKey::IsPaused, &pause);
    }
}

fn assert_vault_active(env: &Env) {
    let is_paused: bool = env.storage().instance().get(&DataKey::IsPaused).unwrap_or(false);
    assert!(!is_paused, "Vault is currently paused");
}