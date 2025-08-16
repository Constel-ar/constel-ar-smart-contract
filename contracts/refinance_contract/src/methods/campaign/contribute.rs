use soroban_sdk::{Address, Env, String};

use crate::{events::add_contribute, methods::{campaign::add_campaign::add_campaign, token::token_transfer}, storage::{campaign::{get_campaign, set_campaign}, contribution::{get_contribution, set_contribution}, structs::contribution::Contribution, types::{campaign_state::CampaignState, error::Error}}};

pub fn contribute (
    env: Env,
    sender: Address,
    amount: i128,
    campaign_id: String,
) -> Result<(), Error>{
    
    if amount <= 0 {
        return Err(Error::AmountMustBePositive);
    }

    sender.require_auth();
    
    let mut campaign = get_campaign(&env, campaign_id.clone())?;

    if campaign.state != CampaignState::RUNNING {
        return Err(Error::CampaignNotRunning);
    }

    if amount < campaign.min_donation {
        return Err(Error::ContributionBelowMinimum);
    }

    let remaining = campaign.goal.checked_sub(campaign.total_raised).ok_or(Error::MathUnderflow)?;
    if remaining <= 0 {
        return Err(Error::CampaignGoalExceeded);
    }

    let contribution_amount = amount.min(remaining);

    token_transfer(
        &env, 
        &sender, 
        &env.current_contract_address(), 
        &contribution_amount
    )?;

    let mut prev = get_contribution(&env, campaign_id.clone(), sender.clone());

    let previous_contribution = match prev {
        None => 0,
        Some(contribution) => contribution.amount
    };

    let new_total = previous_contribution.checked_add(contribution_amount).ok_or(Error::MathOverflow)?;
    
    let new_state = Contribution {
        amount: new_total
    };

    set_contribution(&env, campaign_id.clone(), sender.clone(), new_state);

    if previous_contribution == 0 {
        campaign.supporters = campaign.supporters.checked_add(1).ok_or(Error::MathOverflow)?;
    }

    
    campaign.total_raised = campaign.total_raised.checked_add(contribution_amount).ok_or(Error::MathOverflow)?;
    if campaign.total_raised == campaign.goal { campaign.state = CampaignState::COMPLETE; }
    
    
    set_campaign(&env, campaign_id.clone(), campaign.clone());

    crate::methods::campaign::contribute::add_contribute::contribution_added(&env, &sender, campaign_id.clone(), &amount);

    Ok(())
}