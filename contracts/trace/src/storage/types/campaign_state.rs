use soroban_sdk::contracttype;

#[derive(Clone, PartialEq, PartialOrd)]
#[contracttype]
pub enum CampaignState {
    RUNNING,
    COMPLETE,
}
