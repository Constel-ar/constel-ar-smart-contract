use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum CampaignState {
    RUNNING,
    COMPLETE,
}
