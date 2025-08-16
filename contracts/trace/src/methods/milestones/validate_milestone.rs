use soroban_sdk::{Env, String};

use crate::{
    events,
    storage::{
        admin::get_admin,
        campaign::{get_campaign, set_campaign},
        milestone::{get_milestone, set_milestone},
        proof::has_proof,
        types::error::Error,
    },
};

pub fn validate_milestone(
    env: &Env,
    campaign_id: String,
    milestone_id: String,
    milestone_sequence: u32,
) -> Result<(), Error> {
    // Only the contract admin can validate milestones.
    let admin = get_admin(env);
    admin.require_auth();

    let mut campaign = get_campaign(env, campaign_id.clone())?;

    // Ensure milestones are validated in sequential order.
    if milestone_sequence != campaign.current_milestone + 1 {
        return Err(Error::MilestoneInvalidSequence);
    }

    // A proof must exist before validation.
    if !has_proof(env, milestone_id.clone()) {
        return Err(Error::ProofNotFound);
    }

    let mut milestone = get_milestone(env, &campaign_id, milestone_sequence)?;
    if milestone.completed {
        return Err(Error::MilestoneAlreadyCompleted);
    }

    // Calculate the amount to release for this specific milestone.
    let amount_to_release = if milestone.sequence == 1 {
        milestone.target_amount
    } else {
        let prev_milestone = get_milestone(env, &campaign_id, milestone.sequence - 1)?;
        milestone
            .target_amount
            .checked_sub(prev_milestone.target_amount)
            .ok_or(Error::MathUnderflow)?
    };

    // Update state
    milestone.completed = true;
    campaign.current_milestone = milestone_sequence;
    campaign.withdrawable_amount = campaign
        .withdrawable_amount
        .checked_add(amount_to_release)
        .ok_or(Error::MathOverflow)?;

    // Save updated state to storage
    set_milestone(env, &campaign_id, milestone_sequence, &milestone);
    set_campaign(env, &campaign_id, &campaign);

    // Emit events
    events::milestone::milestone_completed(
        env,
        campaign_id.clone(),
        milestone_sequence,
        milestone_id,
    );

    Ok(())
}
