use soroban_sdk::{Address, Env, String};

use crate::{
    events::{
        refund::user_refunded
    }, methods::{
        token::token_transfer
    }, storage::{
        campaign::{
            get_campaign, 
            set_campaign
        }, contribution::{
            get_contribution, 
            remove_contribution,
        }, types::{
            campaign_state::CampaignState, 
            error::Error
        }
    }
};

pub fn refund (env: &Env, contributor: Address, campaign_id: String) -> Result<(), Error> {
    contributor.require_auth();

    let mut campaign = get_campaign(env, campaign_id.clone())?;

    if campaign.state == CampaignState::COMPLETE {
        return Err(Error::CampaignNorRefundable)
    }

    let prev = get_contribution(&env, campaign_id.clone(), contributor.clone());

    let previous_contribution = prev.ok_or(Error::ContributionNotFound)?;
    
    // Validate contribution amount is positive
    if previous_contribution.amount <= 0 {
        return Err(Error::ContributionNotFound);
    }
    
    token_transfer(env, &env.current_contract_address(), &contributor, &previous_contribution.amount)?;
    
    // Atomic state updates to prevent inconsistencies
    campaign.total_raised = campaign.total_raised.checked_sub(previous_contribution.amount).ok_or(Error::MathUnderflow)?;
    
    // Only decrement supporters if this was their only contribution
    if previous_contribution.amount > 0 {
        campaign.supporters = campaign.supporters.saturating_sub(1);
    }
    
    remove_contribution(env, campaign_id.clone(), contributor.clone());
    set_campaign(env, campaign_id.clone(), campaign);

    user_refunded(env, &contributor, campaign_id, &previous_contribution.amount);

    Ok(())
}