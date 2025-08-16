use soroban_sdk::{Env, String};

use crate::storage::{
    structs::campaign::Campaign,
    types::{error::Error, storage::DataKey},
};

pub(crate) fn campaign_key(campaign_id: &String) -> DataKey {
    DataKey::Campaign(campaign_id.clone())
}

pub(crate) fn has_campaign(env: &Env, campaign_id: &String) -> bool {
    let key = campaign_key(campaign_id);
    env.storage().persistent().has(&key)
}

pub(crate) fn create_campaign(env: &Env, campaign_id: String, campaign: Campaign) {
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
