use crate::tests::config::{contract::ContractTest, utils::get_contract_events};
use soroban_sdk::{vec, IntoVal, Symbol};

#[test]
fn deploy_test() {
    let ContractTest {
        env,
        admin,
        contract,
        //user_a,
        ..
    } = ContractTest::setup();
    //contract.mock_all_auths().set_admin(&user_a);

    let contract_events = get_contract_events(&env, contract.address.clone());

    assert_eq!(
        contract_events,
        vec![
            &env,
            (
                contract.address.clone(),
                (Symbol::new(&env, "contract_initialized"), admin.clone()).into_val(&env),
                admin.clone().into_val(&env),
            )
        ]
    );
}
