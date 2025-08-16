use soroban_sdk::{contracttype, String};

#[derive(Clone)]
#[contracttype]
pub struct Milestone {
    pub campaign_id: String,
    pub milestone_id: String,
    pub target_amount: i128,
    pub completed: bool,
    pub sequence: u32,
}
