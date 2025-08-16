use soroban_sdk::{Address, Env, String};

use crate::storage::{
    admin::get_admin,
    campaign::{has_campaign, set_campaign},
    structs::campaign::Campaign,
    types::{campaign_state::CampaignState, error::Error},
};

pub fn add_campaign(
    env: &Env,
    campaign_id: String,
    creator: Address,
    goal: i128,
    min_donation: i128,
) -> Result<(), Error> {
    let admin = get_admin(env);

    admin.require_auth();

    if goal <= 0 {
        return Err(Error::InvalidGoalAmount);
    }

    if min_donation <= 0 || min_donation > goal {
        return Err(Error::InvalidMinDonation);
    }

    // Check if campaign already exists
    if has_campaign(env, &campaign_id) {
        return Err(Error::CampaignAlreadyExists);
    }

    let campaign = Campaign {
        campaign_id: campaign_id.clone(),
        owner: creator,
        goal,
        min_donation,
        total_raised: 0,
        supporters: 0,
        state: CampaignState::RUNNING,

        milestones_count: 0,
        current_milestone: 0,
        withdrawable_amount: 0,
    };

    set_campaign(env, &campaign_id, &campaign);

    Ok(())
}
