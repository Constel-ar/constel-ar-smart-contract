use soroban_sdk::{contracttype, Address, String};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Token,
    Contributions(Address, String), // User, campaign_id
    Campaign(String), // campaign_id
    Milestone(String), // campaign_id
    Proof(String) // milestone_id
}

impl DataKey {
    /// Optimized method to check if a key is for contributions
    pub fn is_contribution(&self) -> bool {
        matches!(self, DataKey::Contributions(_, _))
    }
    
    /// Optimized method to check if a key is for campaigns
    pub fn is_campaign(&self) -> bool {
        matches!(self, DataKey::Campaign(_))
    }
}