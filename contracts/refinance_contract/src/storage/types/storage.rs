use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Campaign(Address), // campaign_id
    Milestone(Address), // campaign_id
    Proof(Address) // milestone_id
}