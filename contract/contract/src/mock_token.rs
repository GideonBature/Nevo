use soroban_sdk::{contract, contractimpl, Address, Env, String};

use crate::base::{
    errors::CrowdfundingError,
    events,
    types::StorageKey,
};

#[contract]
pub struct MockTokenContract;

#[contractimpl]
impl MockTokenContract {
    /// Initialize the token with name, symbol, and decimals
    pub fn initialize(
        env: Env,
        admin: Address,
        name: String,
        symbol: String,
        decimals: u32,
    ) -> Result<(), CrowdfundingError> {
        if env.storage().instance().has(&StorageKey::Admin) {
            return Err(CrowdfundingError::ContractAlreadyInitialized);
        }

        env.storage().instance().set(&StorageKey::Admin, &admin);
        env.storage().instance().set(&StorageKey::TokenName, &name);
        env.storage().instance().set(&StorageKey::TokenSymbol, &symbol);
        env.storage().instance().set(&StorageKey::TokenDecimals, &decimals);
        env.storage().instance().set(&StorageKey::TotalSupply, &0i128);

        Ok(())
    }

    /// Mint tokens to an address (admin only)
    pub fn mint(
        env: Env,
        to: Address,
        amount: i128,
    ) -> Result<(), CrowdfundingError> {
        // Validate amount
        if amount <= 0 {
            return Err(CrowdfundingError::InvalidAmount);
        }

        // Check admin authorization
        let admin: Address = env
            .storage()
            .instance()
            .get(&StorageKey::Admin)
            .ok_or(CrowdfundingError::UnauthorizedMinting)?;
        admin.require_auth();

        // Update total supply
        let current_supply: i128 = env
            .storage()
            .instance()
            .get(&StorageKey::TotalSupply)
            .unwrap_or(0);
        let new_supply = current_supply.checked_add(amount)
            .ok_or(CrowdfundingError::InvalidAmount)?;
        env.storage().instance().set(&StorageKey::TotalSupply, &new_supply);

        // Update recipient balance
        let balance_key = to.clone();
        let current_balance: i128 = env
            .storage()
            .instance()
            .get(&balance_key)
            .unwrap_or(0);
        let new_balance = current_balance.checked_add(amount)
            .ok_or(CrowdfundingError::InvalidAmount)?;
        env.storage().instance().set(&balance_key, &new_balance);

        // Emit mint event
        events::token_minted(&env, to, amount, new_supply);

        Ok(())
    }

    /// Transfer tokens between addresses
    pub fn transfer(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), CrowdfundingError> {
        // Validate amount
        if amount <= 0 {
            return Err(CrowdfundingError::InvalidAmount);
        }

        from.require_auth();

        // Check sender balance
        let from_balance_key = from.clone();
        let from_balance: i128 = env
            .storage()
            .instance()
            .get(&from_balance_key)
            .unwrap_or(0);

        if from_balance < amount {
            return Err(CrowdfundingError::InsufficientBalance);
        }

        // Update sender balance
        let new_from_balance = from_balance.checked_sub(amount)
            .ok_or(CrowdfundingError::InvalidAmount)?;
        env.storage().instance().set(&from_balance_key, &new_from_balance);

        // Update recipient balance
        let to_balance_key = to.clone();
        let to_balance: i128 = env
            .storage()
            .instance()
            .get(&to_balance_key)
            .unwrap_or(0);
        let new_to_balance = to_balance.checked_add(amount)
            .ok_or(CrowdfundingError::InvalidAmount)?;
        env.storage().instance().set(&to_balance_key, &new_to_balance);

        // Emit transfer event
        events::token_transferred(&env, from, to, amount);

        Ok(())
    }

    /// Get balance of an address
    pub fn balance(env: Env, address: Address) -> i128 {
        env.storage()
            .instance()
            .get(&address)
            .unwrap_or(0)
    }

    /// Get total token supply
    pub fn total_supply(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&StorageKey::TotalSupply)
            .unwrap_or(0)
    }

    /// Get token name
    pub fn name(env: Env) -> String {
        env.storage()
            .instance()
            .get(&StorageKey::TokenName)
            .unwrap_or(String::from_str(&env, ""))
    }

    /// Get token symbol
    pub fn symbol(env: Env) -> String {
        env.storage()
            .instance()
            .get(&StorageKey::TokenSymbol)
            .unwrap_or(String::from_str(&env, ""))
    }

    /// Get token decimals
    pub fn decimals(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&StorageKey::TokenDecimals)
            .unwrap_or(0)
    }
}