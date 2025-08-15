use soroban_sdk::{Env, String};

use crate::storage::{structs::campaign::Campaign, types::storage::DataKey};

pub(crate) fn create_campaign(env: &Env, campaign_id: String, campaign: Campaign) {
    let key = DataKey::Campaign(campaign_id);

    env.storage().instance().set(&key, &campaign)
}

pub(crate) fn get_campaign(env: &Env, campaign_id: String) -> Option<Campaign> {
    let key = DataKey::Campaign(campaign_id);

    env.storage().instance().get(&key)
}
