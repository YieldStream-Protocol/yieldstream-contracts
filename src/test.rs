#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};
use soroban_sdk::token::{Client as TokenClient, StellarAssetClient};

fn create_token<'a>(env: &Env, admin: &Address) -> (TokenClient<'a>, StellarAssetClient<'a>) {
    let contract_id = env.register_stellar_asset_contract(admin.clone());
    (
        TokenClient::new(env, &contract_id),
        StellarAssetClient::new(env, &contract_id),
    )
}

#[test]
fn test_vault_lifecycle() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let keeper = Address::generate(&env);

    // Setup Mock Tokens
    let (underlying, underlying_admin) = create_token(&env, &admin);
    let (share, share_admin) = create_token(&env, &admin);

    // Deploy Vault
    let contract_id = env.register_contract(None, YieldStreamVault);
    let client = YieldStreamVaultClient::new(&env, &contract_id);

    // Vault needs admin rights to mint share tokens
    share_admin.set_admin(&contract_id);

    // Initialize Vault
    client.initialize(&admin, &underlying.address, &share.address);

    // Mint underlying to user and keeper (for simulated yield)
    underlying_admin.mint(&user, &1000);
    underlying_admin.mint(&keeper, &100);

    // User deposits 1000
    let shares = client.deposit(&user, &1000);
    assert_eq!(shares, 1000); // 1:1 on first deposit
    assert_eq!(underlying.balance(&contract_id), 1000);

    // Keeper harvests 100 yield (Vault TVL goes to 1100, Total Shares stay 1000)
    client.harvest_yield(&keeper, &100);
    assert_eq!(underlying.balance(&contract_id), 1100);

    // User withdraws 500 shares (50% of pool)
    // Should receive: (500 * 1100) / 1000 = 550 underlying tokens
    let withdrawn = client.withdraw(&user, &500);
    assert_eq!(withdrawn, 550);
    assert_eq!(underlying.balance(&user), 550);
    assert_eq!(underlying.balance(&contract_id), 550); // Vault retains the other 50%
}