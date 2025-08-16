use soroban_sdk::{Address, Env, String};

use super::types::storage::DataKey;

pub(crate) fn has_contribution(env: &Env, campaign_id: &String, contributor: &Address) -> bool {
    let key = DataKey::Contribution(campaign_id.clone(), contributor.clone());

    env.storage().persistent().has(&key)
}

pub(crate) fn set_contribution(
    env: &Env,
    campaign_id: &String,
    contributor: &Address,
    amount: i128,
) {
    let key = DataKey::Contribution(campaign_id.clone(), contributor.clone());

    env.storage().persistent().set(&key, &amount);
}

pub(crate) fn get_contribution(env: &Env, campaign_id: &String, contributor: &Address) -> i128 {
    let key = DataKey::Contribution(campaign_id.clone(), contributor.clone());

    env.storage().persistent().get(&key).unwrap_or(0)
}

pub(crate) fn remove_contribution(env: &Env, campaign_id: &String, contributor: &Address) {
    let key = DataKey::Contribution(campaign_id.clone(), contributor.clone());

    env.storage().persistent().remove(&key);
}
