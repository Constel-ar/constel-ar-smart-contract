use soroban_sdk::{contracttype, String};

#[derive(Clone)]
#[contracttype]
pub struct Proof {
    pub milestone_id: String,
    pub uri: String
}