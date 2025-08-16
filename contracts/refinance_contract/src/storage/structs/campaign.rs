use soroban_sdk::{contracttype, Address, String};

use crate::storage::types::campaign_state::CampaignState;

#[derive(Clone)]
#[contracttype]
pub struct Campaign {
    pub campaign_id: String, // Campaign identifier
    pub owner: Address,
    pub goal: i128,
    pub min_donation: i128,
    pub total_raised: i128,
    pub supporters: u32,
    pub state: CampaignState,

    // validation
    pub milestones_count: u32,     // Total milestones for this campaign
    pub current_milestone: u32,    // Latest completed milestone (0 = none)
    pub withdrawable_amount: i128, // Amount available for withdrawal
}
