#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Env, IntoVal, String,
};

use crate::{
    base::errors::CrowdfundingError,
    mock_token::{MockTokenContract, MockTokenContractClient},
};

#[test]
fn test_initialize_token() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MockTokenContract, ());
    let client = MockTokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let name = String::from_str(&env, "Mock Token");
    let symbol = String::from_str(&env, "MOCK");
    let decimals = 18u32;

    client.initialize(&admin, &name, &symbol, &decimals);

    assert_eq!(client.name(), name);
    assert_eq!(client.symbol(), symbol);
    assert_eq!(client.decimals(), decimals);
    assert_eq!(client.total_supply(), 0);
}

#[test]
fn test_initialize_token_twice_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MockTokenContract, ());
    let client = MockTokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let name = String::from_str(&env, "Mock Token");
    let symbol = String::from_str(&env, "MOCK");
    let decimals = 18u32;

    client.initialize(&admin, &name, &symbol, &decimals);

    let result = client.try_initialize(&admin, &name, &symbol, &decimals);
    assert_eq!(result, Err(Ok(CrowdfundingError::ContractAlreadyInitialized)));
}

#[test]
fn test_mint_tokens() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MockTokenContract, ());
    let client = MockTokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let recipient = Address::generate(&env);
    let name = String::from_str(&env, "Mock Token");
    let symbol = String::from_str(&env, "MOCK");
    let decimals = 18u32;

    client.initialize(&admin, &name, &symbol, &decimals);

    let mint_amount = 1000i128;
    client.mint(&recipient, &mint_amount);

    assert_eq!(client.balance(&recipient), mint_amount);
    assert_eq!(client.total_supply(), mint_amount);
}

#[test]
fn test_mint_multiple_times() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MockTokenContract, ());
    let client = MockTokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let recipient1 = Address::generate(&env);
    let recipient2 = Address::generate(&env);
    let name = String::from_str(&env, "Mock Token");
    let symbol = String::from_str(&env, "MOCK");
    let decimals = 18u32;

    client.initialize(&admin, &name, &symbol, &decimals);

    let amount1 = 500i128;
    let amount2 = 750i128;

    client.mint(&recipient1, &amount1);
    client.mint(&recipient2, &amount2);

    assert_eq!(client.balance(&recipient1), amount1);
    assert_eq!(client.balance(&recipient2), amount2);
    assert_eq!(client.total_supply(), amount1 + amount2);
}

#[test]
fn test_mint_zero_amount_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MockTokenContract, ());
    let client = MockTokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let recipient = Address::generate(&env);
    let name = String::from_str(&env, "Mock Token");
    let symbol = String::from_str(&env, "MOCK");
    let decimals = 18u32;

    client.initialize(&admin, &name, &symbol, &decimals);

    let result = client.try_mint(&recipient, &0i128);
    assert_eq!(result, Err(Ok(CrowdfundingError::InvalidAmount)));
}

#[test]
fn test_mint_negative_amount_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MockTokenContract, ());
    let client = MockTokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let recipient = Address::generate(&env);
    let name = String::from_str(&env, "Mock Token");
    let symbol = String::from_str(&env, "MOCK");
    let decimals = 18u32;

    client.initialize(&admin, &name, &symbol, &decimals);

    let result = client.try_mint(&recipient, &-100i128);
    assert_eq!(result, Err(Ok(CrowdfundingError::InvalidAmount)));
}

#[test]
#[should_panic]
fn test_mint_unauthorized() {
    let env = Env::default();
    let contract_id = env.register(MockTokenContract, ());
    let client = MockTokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let non_admin = Address::generate(&env);
    let recipient = Address::generate(&env);
    let name = String::from_str(&env, "Mock Token");
    let symbol = String::from_str(&env, "MOCK");
    let decimals = 18u32;

    client.initialize(&admin, &name, &symbol, &decimals);

    // Try to mint without admin auth - should fail
    client
        .mock_auths(&[soroban_sdk::testutils::MockAuth {
            address: &non_admin,
            invoke: &soroban_sdk::testutils::MockAuthInvoke {
                contract: &contract_id,
                fn_name: "mint",
                args: soroban_sdk::vec![&env, recipient.into_val(&env), 100i128.into_val(&env)],
                sub_invokes: &[],
            },
        }])
        .mint(&recipient, &100i128);
}

#[test]
fn test_transfer_tokens() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MockTokenContract, ());
    let client = MockTokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);
    let name = String::from_str(&env, "Mock Token");
    let symbol = String::from_str(&env, "MOCK");
    let decimals = 18u32;

    client.initialize(&admin, &name, &symbol, &decimals);

    let initial_amount = 1000i128;
    let transfer_amount = 300i128;

    client.mint(&sender, &initial_amount);
    client.transfer(&sender, &recipient, &transfer_amount);

    assert_eq!(client.balance(&sender), initial_amount - transfer_amount);
    assert_eq!(client.balance(&recipient), transfer_amount);
    assert_eq!(client.total_supply(), initial_amount);
}

#[test]
fn test_transfer_zero_amount_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MockTokenContract, ());
    let client = MockTokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);
    let name = String::from_str(&env, "Mock Token");
    let symbol = String::from_str(&env, "MOCK");
    let decimals = 18u32;

    client.initialize(&admin, &name, &symbol, &decimals);

    let initial_amount = 1000i128;
    client.mint(&sender, &initial_amount);

    let result = client.try_transfer(&sender, &recipient, &0i128);
    assert_eq!(result, Err(Ok(CrowdfundingError::InvalidAmount)));
}

#[test]
fn test_transfer_negative_amount_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MockTokenContract, ());
    let client = MockTokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);
    let name = String::from_str(&env, "Mock Token");
    let symbol = String::from_str(&env, "MOCK");
    let decimals = 18u32;

    client.initialize(&admin, &name, &symbol, &decimals);

    let initial_amount = 1000i128;
    client.mint(&sender, &initial_amount);

    let result = client.try_transfer(&sender, &recipient, &-100i128);
    assert_eq!(result, Err(Ok(CrowdfundingError::InvalidAmount)));
}

#[test]
fn test_transfer_insufficient_balance_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MockTokenContract, ());
    let client = MockTokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);
    let name = String::from_str(&env, "Mock Token");
    let symbol = String::from_str(&env, "MOCK");
    let decimals = 18u32;

    client.initialize(&admin, &name, &symbol, &decimals);

    let initial_amount = 500i128;
    let transfer_amount = 1000i128;

    client.mint(&sender, &initial_amount);

    let result = client.try_transfer(&sender, &recipient, &transfer_amount);
    assert_eq!(result, Err(Ok(CrowdfundingError::InsufficientBalance)));
}

#[test]
fn test_transfer_to_self() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MockTokenContract, ());
    let client = MockTokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let name = String::from_str(&env, "Mock Token");
    let symbol = String::from_str(&env, "MOCK");
    let decimals = 18u32;

    client.initialize(&admin, &name, &symbol, &decimals);

    let initial_amount = 1000i128;
    client.mint(&user, &initial_amount);

    // Transfer to self should work (no change in balance)
    client.transfer(&user, &user, &initial_amount);

    assert_eq!(client.balance(&user), initial_amount);
    assert_eq!(client.total_supply(), initial_amount);
}

#[test]
fn test_balance_of_zero_address() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MockTokenContract, ());
    let client = MockTokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let zero_address = Address::generate(&env);
    let name = String::from_str(&env, "Mock Token");
    let symbol = String::from_str(&env, "MOCK");
    let decimals = 18u32;

    client.initialize(&admin, &name, &symbol, &decimals);

    // Address with no tokens should have zero balance
    assert_eq!(client.balance(&zero_address), 0);
}

#[test]
fn test_multiple_transfers() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MockTokenContract, ());
    let client = MockTokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let user3 = Address::generate(&env);
    let name = String::from_str(&env, "Mock Token");
    let symbol = String::from_str(&env, "MOCK");
    let decimals = 18u32;

    client.initialize(&admin, &name, &symbol, &decimals);

    // Mint to user1
    let amount1 = 2000i128;
    client.mint(&user1, &amount1);

    // Transfer from user1 to user2
    let transfer1 = 500i128;
    client.transfer(&user1, &user2, &transfer1);

    // Transfer from user2 to user3
    let transfer2 = 200i128;
    client.transfer(&user2, &user3, &transfer2);

    // Transfer from user1 to user3
    let transfer3 = 300i128;
    client.transfer(&user1, &user3, &transfer3);

    assert_eq!(client.balance(&user1), amount1 - transfer1 - transfer3);
    assert_eq!(client.balance(&user2), transfer1 - transfer2);
    assert_eq!(client.balance(&user3), transfer2 + transfer3);
    assert_eq!(client.total_supply(), amount1);
}

#[test]
fn test_total_supply_increases_with_mint() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MockTokenContract, ());
    let client = MockTokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let name = String::from_str(&env, "Mock Token");
    let symbol = String::from_str(&env, "MOCK");
    let decimals = 18u32;

    client.initialize(&admin, &name, &symbol, &decimals);

    assert_eq!(client.total_supply(), 0);

    client.mint(&user1, &100i128);
    assert_eq!(client.total_supply(), 100);

    client.mint(&user2, &200i128);
    assert_eq!(client.total_supply(), 300);

    client.mint(&user1, &50i128);
    assert_eq!(client.total_supply(), 350);
}

#[test]
fn test_total_supply_unchanged_by_transfers() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MockTokenContract, ());
    let client = MockTokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let name = String::from_str(&env, "Mock Token");
    let symbol = String::from_str(&env, "MOCK");
    let decimals = 18u32;

    client.initialize(&admin, &name, &symbol, &decimals);

    let total_supply = 1000i128;
    client.mint(&user1, &total_supply);

    // Transfer should not change total supply
    client.transfer(&user1, &user2, &500i128);
    assert_eq!(client.total_supply(), total_supply);

    client.transfer(&user2, &user1, &200i128);
    assert_eq!(client.total_supply(), total_supply);
}