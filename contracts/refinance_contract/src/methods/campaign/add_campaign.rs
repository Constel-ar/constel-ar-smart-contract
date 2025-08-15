use soroban_sdk::{Address, Env, String};

use crate::{
    storage::{
    admin::get_admin, 
    campaign::set_campaign, 
    structs::campaign::Campaign, 
    types::{
        campaign_state::CampaignState, 
        error::Error
    }}};

pub fn add_campaign(
    env: &Env,
    campaign_id: String,
    creator: Address ,
    goal: i128 ,
    min_donation: i128
) -> Result<(), Error> {
    let admin = get_admin(env);

    admin.require_auth();

    let campaign = Campaign{
        campaign_id: campaign_id.clone(),
        owner: creator,
        goal,
        min_donation,
        total_raised: 0,
        supporters: 0,
        state: CampaignState::RUNNING
    };

    set_campaign(env, campaign_id, campaign);

    Ok(())
}