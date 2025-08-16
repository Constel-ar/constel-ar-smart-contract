use crate::events;
use crate::storage::milestone::get_milestone;
use crate::storage::types::error::Error;
use crate::storage::{self, structs::milestone::*};
use soroban_sdk::{Env, String, Vec};

pub fn add_milestone(
    env: &Env,
    campaign_id: String,
    milestone_id: String,
    target_amount: i128,
) -> Result<(), Error> {
    // Verify campaign exists and get it
    let campaign = storage::campaign::get_campaign(env, campaign_id.clone())?;

    // Verify creator authorization
    campaign.owner.require_auth();

    // Validate target amount
    if target_amount <= 0 || target_amount > campaign.goal {
        return Err(Error::InvalidMilestoneAmount);
    }

    // Get next sequence number
    let sequence = campaign.milestones_count + 1;

    // Validate sequential ordering (each milestone should be higher than previous)
    if sequence > 1 {
        let prev_milestone = get_milestone(env, &campaign_id.clone(), sequence - 1)?;
        if target_amount <= prev_milestone.target_amount {
            return Err(Error::MilestoneAmountNotIncreasing);
        }
    }

    // Create milestone
    let milestone = Milestone {
        campaign_id: campaign_id.clone(),
        milestone_id: milestone_id.clone(),
        completed: false,
        sequence,
        target_amount,
    };

    // Store milestone
    storage::milestone::set_milestone(env, &campaign_id, sequence, &milestone);

    // Update campaign milestone count
    let mut updated_campaign = campaign;
    updated_campaign.milestones_count = sequence;
    storage::campaign::set_campaign(env, &campaign_id, &updated_campaign);

    // Emit event
    events::milestone::milestone_created(env, campaign_id.clone(), sequence, target_amount);

    Ok(())
}
