#![cfg(test)]

use soroban_sdk::symbol_short;
use soroban_sdk::testutils::{storage::Instance as _, Ledger};

use super::{WalletBinding, WalletContract, WalletContractClient};
use lily_common::{INSTANCE_BUMP_AMOUNT, INSTANCE_BUMP_THRESHOLD};
use lily_test_support::{test_address, test_env};

#[test]
fn mutations_refresh_instance_ttl() {
    let env = test_env();
    let admin = test_address(&env);
    let agent = test_address(&env);
    let wallet = test_address(&env);
    let contract_id = env.register(WalletContract, ());
    let client = WalletContractClient::new(&env, &contract_id);

    client.initialize(&admin);
    env.ledger().set_sequence_number(INSTANCE_BUMP_AMOUNT - INSTANCE_BUMP_THRESHOLD + 1);
    let ttl_before = env.as_contract(&contract_id, || env.storage().instance().get_ttl());
    assert!(ttl_before < INSTANCE_BUMP_THRESHOLD);

    client.bind_wallet(&agent, &wallet, &symbol_short!("USDC"), &1_000_i128);

    let ttl_after = env.as_contract(&contract_id, || env.storage().instance().get_ttl());
    assert_eq!(ttl_after, INSTANCE_BUMP_AMOUNT);
}

#[test]
fn binds_wallet_and_updates_policy() {
    let env = test_env();
    let admin = test_address(&env);
    let agent = test_address(&env);
    let wallet = test_address(&env);

    let contract_id = env.register(WalletContract, ());
    let client = WalletContractClient::new(&env, &contract_id);

    client.initialize(&admin);
    client.bind_wallet(&agent, &wallet, &symbol_short!("USDC"), &1_000_i128);

    let binding = client.get_binding(&agent);
    assert_eq!(
        binding,
        WalletBinding {
            wallet: wallet.clone(),
            settlement_asset: symbol_short!("USDC"),
            spend_limit: 1_000,
            enabled: true,
            revision: 0,
        }
    );

    client.update_spend_limit(&agent, &2_500_i128);
    client.set_enabled(&agent, &false);

    let updated = client.get_binding(&agent);
    assert_eq!(updated.spend_limit, 2_500);
    assert!(!updated.enabled);
    assert_eq!(updated.revision, 2);
}

#[test]
#[should_panic]
fn rejects_double_binding_while_active() {
    let env = test_env();
    let admin = test_address(&env);
    let agent = test_address(&env);
    let wallet = test_address(&env);

    let contract_id = env.register(WalletContract, ());
    let client = WalletContractClient::new(&env, &contract_id);

    client.initialize(&admin);
    client.bind_wallet(&agent, &wallet, &symbol_short!("USDC"), &100_i128);
    client.bind_wallet(&agent, &wallet, &symbol_short!("USDC"), &100_i128);
}

#[test]
#[should_panic]
fn rejects_zero_spend_limit() {
    let env = test_env();
    let admin = test_address(&env);
    let agent = test_address(&env);
    let wallet = test_address(&env);

    let contract_id = env.register(WalletContract, ());
    let client = WalletContractClient::new(&env, &contract_id);

    client.initialize(&admin);
    client.bind_wallet(&agent, &wallet, &symbol_short!("USDC"), &0_i128);
}
