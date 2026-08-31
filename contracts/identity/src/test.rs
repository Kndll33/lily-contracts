#![cfg(test)]

use super::{AgentProfile, IdentityContract, IdentityContractClient};
use lily_common::{INSTANCE_BUMP_AMOUNT, INSTANCE_BUMP_THRESHOLD};
use lily_test_support::{soroban_string, test_address, test_env};
use soroban_sdk::testutils::{storage::Instance as _, Ledger};

#[test]
fn mutations_refresh_instance_ttl() {
    let env = test_env();
    let admin = test_address(&env);
    let agent = test_address(&env);
    let controller = test_address(&env);
    let contract_id = env.register(IdentityContract, ());
    let client = IdentityContractClient::new(&env, &contract_id);

    client.initialize(&admin);
    env.ledger().set_sequence_number(INSTANCE_BUMP_AMOUNT - INSTANCE_BUMP_THRESHOLD + 1);
    let ttl_before = env.as_contract(&contract_id, || env.storage().instance().get_ttl());
    assert!(ttl_before < INSTANCE_BUMP_THRESHOLD);

    client.register(&agent, &controller, &soroban_string(&env, "ipfs://profile"));

    let ttl_after = env.as_contract(&contract_id, || env.storage().instance().get_ttl());
    assert_eq!(ttl_after, INSTANCE_BUMP_AMOUNT);
}

#[test]
fn registers_and_updates_profiles() {
    let env = test_env();
    let admin = test_address(&env);
    let agent = test_address(&env);
    let controller = test_address(&env);
    let new_controller = test_address(&env);

    let contract_id = env.register(IdentityContract, ());
    let client = IdentityContractClient::new(&env, &contract_id);

    client.initialize(&admin);
    client.register(&agent, &controller, &soroban_string(&env, "ipfs://agent-lily/profile-v1"));

    let profile = client.get_profile(&agent);
    assert_eq!(
        profile,
        AgentProfile {
            controller: controller.clone(),
            metadata_uri: soroban_string(&env, "ipfs://agent-lily/profile-v1"),
            active: true,
            revision: 0,
        }
    );

    client.update_profile(
        &agent,
        &soroban_string(&env, "ipfs://agent-lily/profile-v2"),
        &Some(new_controller.clone()),
    );

    let updated = client.get_profile(&agent);
    assert_eq!(updated.controller, new_controller);
    assert_eq!(updated.revision, 1);
}

#[test]
#[should_panic]
fn rejects_duplicate_registration() {
    let env = test_env();
    let admin = test_address(&env);
    let agent = test_address(&env);
    let controller = test_address(&env);

    let contract_id = env.register(IdentityContract, ());
    let client = IdentityContractClient::new(&env, &contract_id);

    client.initialize(&admin);
    client.register(&agent, &controller, &soroban_string(&env, "ipfs://profile"));
    client.register(&agent, &controller, &soroban_string(&env, "ipfs://profile"));
}

#[test]
fn admin_can_deactivate_profiles() {
    let env = test_env();
    let admin = test_address(&env);
    let agent = test_address(&env);
    let controller = test_address(&env);

    let contract_id = env.register(IdentityContract, ());
    let client = IdentityContractClient::new(&env, &contract_id);

    client.initialize(&admin);
    client.register(&agent, &controller, &soroban_string(&env, "ipfs://profile"));
    client.deactivate(&agent);

    let profile = client.get_profile(&agent);
    assert!(!profile.active);
    assert_eq!(profile.revision, 1);
}
