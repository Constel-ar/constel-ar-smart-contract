use soroban_sdk::{Address, Env, String, Symbol};

pub(crate) fn add_contribute(
    env: &Env,
    contributor: &Address,
    campaign_id: &String,
    amount: &i128,
) {
    let topics = (Symbol::new(env, "add_contribute"), contributor);
    let data = (campaign_id.clone(), amount);
    env.events().publish(topics, data);
}
