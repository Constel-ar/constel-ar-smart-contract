use soroban_sdk::{Env, String};

use crate::storage::{
    structs::campaign::Campaign,
    types::{campaign_state::CampaignState, error::Error, storage::DataKey},
};

pub(crate) fn set_campaign(env: &Env, campaign_id: String, campaign: Campaign) {
    let key = DataKey::Campaign(campaign_id);

    env.storage().instance().set(&key, &campaign)
}

pub(crate) fn get_campaign(env: &Env, campaign_id: String) -> Result<Campaign, Error> {
    let key = DataKey::Campaign(campaign_id);

    env.storage()
        .instance()
        .get(&key)
        .ok_or(Error::CampaignNotFound)
}
