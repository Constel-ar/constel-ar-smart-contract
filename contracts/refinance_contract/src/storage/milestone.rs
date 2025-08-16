use crate::storage::{
    structs::milestone::Milestone,
    types::{error::Error, storage::DataKey},
};
use soroban_sdk::{Env, String};

pub(crate) fn milestone_key(campaign_id: &String, sequence: u32) -> DataKey {
    DataKey::Milestone(campaign_id.clone(), sequence)
}

pub(crate) fn set_milestone(env: &Env, campaign_id: &String, sequence: u32, milestone: &Milestone) {
    let key = milestone_key(campaign_id, sequence);
    env.storage().persistent().set(&key, milestone);
}

pub(crate) fn get_milestone(
    env: &Env,
    campaign_id: &String,
    sequence: u32,
) -> Result<Milestone, Error> {
    let key = milestone_key(campaign_id, sequence);
    env.storage()
        .persistent()
        .get(&key)
        .ok_or(Error::MilestoneNotFound)
}

pub(crate) fn has_milestone(env: &Env, campaign_id: &String, sequence: u32) -> bool {
    let key = milestone_key(campaign_id, sequence);
    env.storage().persistent().has(&key)
}

pub(crate) fn remove_milestone(env: &Env, campaign_id: &String, sequence: u32) {
    let key = milestone_key(campaign_id, sequence);
    env.storage().persistent().remove(&key);
}
