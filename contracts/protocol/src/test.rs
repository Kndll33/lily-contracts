#![cfg(test)]

use super::{DataKey, ProtocolConfig, ProtocolContract, ProtocolContractClient};
use lily_test_support::{test_address, test_env};
use soroban_sdk::{FromVal, IntoVal, Symbol, Val, Vec};

#[test]
fn data_key_encodings_are_stable() {
    let env = test_env();

    let cases = [
        (DataKey::Admin, "Admin"),
        (DataKey::Treasury, "Treasury"),
        (DataKey::FeeBps, "FeeBps"),
        (DataKey::Initialized, "Initialized"),
    ];

    for (key, variant) in cases {
        let expected: Vec<Val> = soroban_sdk::vec![&env, Symbol::new(&env, variant).into_val(&env)];
        let actual: Val = key.into_val(&env);
        assert_eq!(Vec::<Val>::from_val(&env, &actual), expected);
    }
}

#[test]
fn initializes_once_and_reads_config() {
    let env = test_env();
    let admin = test_address(&env);
    let treasury = test_address(&env);

    let contract_id = env.register(ProtocolContract, ());
    let client = ProtocolContractClient::new(&env, &contract_id);

    client.initialize(&admin, &treasury, &250_u32);
    assert!(client.is_initialized());

    let config = client.get_config();
    assert_eq!(config, ProtocolConfig { admin, treasury, fee_bps: 250 });
}

#[test]
#[should_panic]
fn rejects_reinitialization() {
    let env = test_env();
    let admin = test_address(&env);
    let treasury = test_address(&env);

    let contract_id = env.register(ProtocolContract, ());
    let client = ProtocolContractClient::new(&env, &contract_id);

    client.initialize(&admin, &treasury, &100_u32);
    client.initialize(&admin, &treasury, &100_u32);
}

#[test]
#[should_panic]
fn rejects_fee_bps_above_max() {
    let env = test_env();
    let admin = test_address(&env);
    let treasury = test_address(&env);

    let contract_id = env.register(ProtocolContract, ());
    let client = ProtocolContractClient::new(&env, &contract_id);

    client.initialize(&admin, &treasury, &10_001_u32);
}

#[test]
fn updates_fee_and_treasury() {
    let env = test_env();
    let admin = test_address(&env);
    let treasury = test_address(&env);
    let new_treasury = test_address(&env);

    let contract_id = env.register(ProtocolContract, ());
    let client = ProtocolContractClient::new(&env, &contract_id);

    client.initialize(&admin, &treasury, &100_u32);
    client.set_fee_bps(&375_u32);
    client.set_treasury(&new_treasury);

    let config = client.get_config();
    assert_eq!(config.fee_bps, 375);
    assert_eq!(config.treasury, new_treasury);
}
