#![cfg(test)]

use lily_common::PaymentStatus;
use lily_test_support::{soroban_string, test_address, test_env};
use soroban_sdk::{FromVal, IntoVal, Symbol, Val, Vec};

use super::{DataKey, PaymentIntent, PaymentsContract, PaymentsContractClient};

#[test]
fn data_key_encodings_are_stable() {
    let env = test_env();

    let scalar_cases = [
        (DataKey::Admin, "Admin"),
        (DataKey::Treasury, "Treasury"),
        (DataKey::FeeBps, "FeeBps"),
        (DataKey::NextIntentId, "NextIntentId"),
        (DataKey::Initialized, "Initialized"),
    ];

    for (key, variant) in scalar_cases {
        let expected: Vec<Val> = soroban_sdk::vec![&env, Symbol::new(&env, variant).into_val(&env)];
        let actual: Val = key.into_val(&env);
        assert_eq!(Vec::<Val>::from_val(&env, &actual), expected);
    }

    let intent: Vec<Val> =
        soroban_sdk::vec![&env, Symbol::new(&env, "Intent").into_val(&env), 42_u64.into_val(&env),];
    let actual_intent: Val = DataKey::Intent(42).into_val(&env);
    assert_eq!(Vec::<Val>::from_val(&env, &actual_intent), intent);
}

#[test]
fn creates_and_settles_payment_intents() {
    let env = test_env();
    let admin = test_address(&env);
    let treasury = test_address(&env);
    let payer = test_address(&env);
    let payee = test_address(&env);

    let contract_id = env.register(PaymentsContract, ());
    let client = PaymentsContractClient::new(&env, &contract_id);

    client.initialize(&admin, &treasury, &50_u32);
    let id = client.create_intent(
        &payer,
        &payee,
        &5_000_i128,
        &soroban_string(&env, "settle agent service fee"),
    );

    assert_eq!(id, 1);
    let intent = client.get_intent(&id);
    assert_eq!(
        intent,
        PaymentIntent {
            id: 1,
            payer_agent: payer.clone(),
            payee_agent: payee.clone(),
            amount: 5_000,
            memo: soroban_string(&env, "settle agent service fee"),
            settlement_reference: soroban_string(&env, ""),
            status: PaymentStatus::Pending,
        }
    );

    client.settle_intent(&id, &soroban_string(&env, "tx-0001"));
    let settled = client.get_intent(&id);
    assert_eq!(settled.status, PaymentStatus::Settled);
    assert_eq!(settled.settlement_reference, soroban_string(&env, "tx-0001"));
}

#[test]
fn payer_can_cancel_pending_intents() {
    let env = test_env();
    let admin = test_address(&env);
    let treasury = test_address(&env);
    let payer = test_address(&env);
    let payee = test_address(&env);

    let contract_id = env.register(PaymentsContract, ());
    let client = PaymentsContractClient::new(&env, &contract_id);

    client.initialize(&admin, &treasury, &50_u32);
    let id = client.create_intent(&payer, &payee, &5_000_i128, &soroban_string(&env, "cancel me"));
    client.cancel_intent(&id);

    let cancelled = client.get_intent(&id);
    assert_eq!(cancelled.status, PaymentStatus::Cancelled);
}

#[test]
#[should_panic]
fn rejects_settle_after_cancellation() {
    let env = test_env();
    let admin = test_address(&env);
    let treasury = test_address(&env);
    let payer = test_address(&env);
    let payee = test_address(&env);

    let contract_id = env.register(PaymentsContract, ());
    let client = PaymentsContractClient::new(&env, &contract_id);

    client.initialize(&admin, &treasury, &50_u32);
    let id = client.create_intent(&payer, &payee, &5_000_i128, &soroban_string(&env, "cancel me"));
    client.cancel_intent(&id);
    client.settle_intent(&id, &soroban_string(&env, "tx-0002"));
}
