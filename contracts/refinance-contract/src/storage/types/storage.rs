use soroban_sdk::{contracttype, Address, String};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Token,
    Contributions(Address, String), // Usuuario, campaign_id
    Campaign(String),               // campaign_id
    Milestone(String),              // campaign_id
    Proof(String),                  // milestone_id
}
