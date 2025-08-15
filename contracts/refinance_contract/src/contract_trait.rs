use soroban_sdk::{Address, Env, Map, String};

use crate::{
    storage::{
        types::{error::Error},
    },
};
pub trait ContractTrait {
    fn __constructor(env: Env, admin: Address, token: Address) -> Result<(), Error>;

    // === CAMPAIGN FUNCTIONS ===
    fn add_campaign(
        env: Env,
        campaign_id: String,
        owner: Address,
        goal: i128,
        min_donation: i128,
    ) -> Result<(), Error>;

    fn add_milestone(
        env: Env,
        campaign_id: String,
        milestone_id: String,
        amount: i128,
    ) -> Result<(), Error>;

    fn batch_add_milestones(
        env: Env,
        campaign_id: String,
        milestones: Map<String, i128>, // (id, amount)
    ) -> Result<(), Error>;

    fn add_proof(
        env: Env,
        milestone_id: String,
        uri: String,
    ) -> Result<(), Error>;

    fn contribute(
        env: Env,
        sender: Address,
        amount: i128,
        campaign_id: String,
    ) -> Result<(), Error>;

    fn refund(env: Env, sender: Address, amount: i128, campaign_id: String) -> Result<(), Error>;
}