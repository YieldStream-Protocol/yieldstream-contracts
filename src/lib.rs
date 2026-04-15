#![no_std]

pub mod strategy;
pub mod vault;
#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    UnderlyingToken,
    ShareToken,
    TotalShares,
    IsPaused,
}

#[contract]
pub struct YieldStreamVault;

#[contractimpl]
impl YieldStreamVault {
    /// Initializes the vault with an admin, underlying asset, and the share token it controls.
    pub fn initialize(
        env: Env,
        admin: Address,
        underlying_token: Address,
        share_token: Address,
    ) {
        assert!(
            !env.storage().instance().has(&DataKey::Admin),
            "Vault already initialized"
        );

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::UnderlyingToken, &underlying_token);
        env.storage().instance().set(&DataKey::ShareToken, &share_token);
        env.storage().instance().set(&DataKey::TotalShares, &0i128);
        env.storage().instance().set(&DataKey::IsPaused, &false);
    }
}