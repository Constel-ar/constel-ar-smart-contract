use soroban_sdk::contracttype;

#[derive(Clone, PartialEq)]
#[contracttype]
pub enum CampaignState {
    RUNNING,
    COMPLETE,
    CANCELLED
}