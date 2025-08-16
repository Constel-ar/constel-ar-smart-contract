use soroban_sdk::{Address, Env, String};

use crate::{
    storage::{
    admin::get_admin, 
    campaign::{set_campaign, get_campaign}, 
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
    let admin = get_admin(env)?;

    admin.require_auth();
    // Input validation
    if goal <= 0 {
        return Err(Error::AmountMustBePositive);
    }
    
    if min_donation <= 0 {
        return Err(Error::AmountMustBePositive);
    }

    // Check if campaign already exists
    if get_campaign(env, campaign_id.clone()).is_ok() {
        return Err(Error::CampaignAlreadyExists);
    }

    admin.require_auth();

    let campaign = Campaign{
        campaign_id: campaign_id.clone(),
        owner: creator,
        goal,
        min_donation,
        total_raised: 0,
        supporters: 0,
        state: CampaignState::RUNNING,
        created_at: env.ledger().timestamp()
    };

    set_campaign(env, campaign_id, campaign);

    Ok(())
}