use soroban_sdk::{contracttype, String};

#[derive(Clone)]
#[contracttype]
pub struct Milestone {
    pub milestone_id: String, // Campaign identifier
    pub amount: i128,
    pub has_been_distributed: bool,
}
