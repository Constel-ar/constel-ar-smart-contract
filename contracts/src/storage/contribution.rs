use soroban_sdk::{Address, Env, String};

use crate::storage::{
    structs::{campaign::Campaign, contribution::Contribution}, 
    types::{error::Error, storage::DataKey}
};

pub(crate) fn set_contribution(env: &Env, campaign_id: &String, user: &Address, contribution: &Contribution) {
    let key = DataKey::Contributions(user.clone(), campaign_id.clone());
    env.storage().instance().set(&key, contribution)
}

pub(crate) fn get_contribution(env: &Env, campaign_id: &String, user: &Address) -> Option<Contribution> {
    let key = DataKey::Contributions(user.clone(), campaign_id.clone());
    env.storage().instance().get(&key)
}

pub(crate) fn remove_contribution(env: &Env, campaign_id: &String, user: &Address) {
    let key = DataKey::Contributions(user.clone(), campaign_id.clone());
    env.storage().instance().remove(&key);
}