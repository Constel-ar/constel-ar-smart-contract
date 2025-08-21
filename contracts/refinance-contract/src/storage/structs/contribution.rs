use soroban_sdk::{contracttype};

#[derive(Clone)]
#[contracttype]
pub struct Contribution {
    pub amount: i128
}