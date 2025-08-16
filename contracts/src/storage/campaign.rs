use soroban_sdk::{Env, String};

use crate::storage::{
    structs::campaign::Campaign, 
    types::{
        error::Error, 
        storage::DataKey, 
        campaign_state::CampaignState
    }
};

pub(crate) fn set_campaign(env: &Env, campaign_id: &String, campaign: &Campaign) {
    let key = DataKey::Campaign(campaign_id.clone());
    env.storage().instance().set(&key, campaign)
}

pub(crate) fn get_campaign(env: &Env, campaign_id: &String) -> Result<Campaign, Error> {
    let key = DataKey::Campaign(campaign_id.clone());
    env.storage().instance().get(&key).ok_or(Error::CampaignNotFound)
}