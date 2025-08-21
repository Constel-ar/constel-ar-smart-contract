use soroban_sdk::{Address, Env, String, Symbol};

pub(crate) fn contribution_added(
    env: &Env,
    contributor: &Address,
    campaign_id: String,
    amount: &i128,
) {
    let topics = (Symbol::new(env, "contribution_added"), contributor);
    let data = (campaign_id, amount);
    env.events().publish(topics, data);
}
