use soroban_sdk::{contracttype, String};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Token,
    Campaign(String), // campaign_id
    Milestone(String), // campaign_id
    Proof(String) // milestone_id
}