#![cfg(test)]

use super::{ProtocolConfig, ProtocolContract, ProtocolContractClient};
use lily_common::{INSTANCE_BUMP_AMOUNT, INSTANCE_BUMP_THRESHOLD};
use lily_test_support::{test_address, test_env};
use soroban_sdk::testutils::{storage::Instance as _, Ledger};

#[test]
fn mutations_refresh_instance_ttl() {
    let env = test_env();
    let admin = test_address(&env);
    let treasury = test_address(&env);
    let contract_id = env.register(ProtocolContract, ());
    let client = ProtocolContractClient::new(&env, &contract_id);

    client.initialize(&admin, &treasury, &50_u32);
    env.ledger().set_sequence_number(INSTANCE_BUMP_AMOUNT - INSTANCE_BUMP_THRESHOLD + 1);
    let ttl_before = env.as_contract(&contract_id, || env.storage().instance().get_ttl());
    assert!(ttl_before < INSTANCE_BUMP_THRESHOLD);

    client.set_fee_bps(&100_u32);

    let ttl_after = env.as_contract(&contract_id, || env.storage().instance().get_ttl());
    assert_eq!(ttl_after, INSTANCE_BUMP_AMOUNT);
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
