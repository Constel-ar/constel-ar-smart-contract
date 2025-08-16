use soroban_sdk::{Address, Env, String, Symbol};

pub(crate) fn user_refunded(env: &Env, contributor: &Address, campaign_id: String, amount: &i128) {
    let topics = (Symbol::new(env, "user_refunded"), contributor);
    let data = (campaign_id, amount);
    env.events().publish(topics, data);
}