use soroban_sdk::{contracttype, Address, String};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Token,
    Campaign(String),       // campaign_id
    Milestone(String, u32), // campaign_id, sequence
    // Milestones,
    Proof(String),                 // milestone_id
    Contribution(String, Address), // (campaign_id, contributor)
}
