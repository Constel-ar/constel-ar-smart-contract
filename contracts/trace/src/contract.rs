use soroban_sdk::{contract, contractimpl, Address, Env, Map, String};

use crate::{
    contract_trait::ContractTrait,
    methods::{
        campaign::{add_campaign::add_campaign, refund::refund, withdraw::withdraw},
        contribute::contribute,
        milestones::{add_milestone::add_milestone, validate_milestone::validate_milestone},
        proofs::proof::add_proof_logic,
        public::initialize::initialize,
    },
    storage::types::error::Error,
};

#[contract]
pub struct Contract;

#[contractimpl]
impl ContractTrait for Contract {
    fn __constructor(env: Env, admin: Address, token: Address) -> Result<(), Error> {
        initialize(&env, admin, token)
    }

    fn add_campaign(
        env: Env,
        campaign_id: String, // TODO: Usar symbol, address, UUID back
        creator: Address,
        goal: i128,
        min_donation: i128,
    ) -> Result<(), Error> {
        add_campaign(&env, campaign_id, creator, goal, min_donation)
    }

    fn add_milestone(
        env: Env,
        campaign_id: String,
        milestone_id: String,
        amount: i128,
    ) -> Result<(), Error> {
        add_milestone(&env, campaign_id, milestone_id, amount)
    }

    fn batch_add_milestones(
        env: Env,
        campaign_id: String,
        milestones: Map<String, i128>,
    ) -> Result<(), Error> {
        for (milestone_id, amount) in milestones.iter() {
            add_milestone(&env, campaign_id.clone(), milestone_id, amount)?;
        }
        Ok(())
    }

    fn add_proof(
        env: Env,
        campaign_id: String,
        milestone_id: String,
        uri: String,
    ) -> Result<(), Error> {
        add_proof_logic(&env, campaign_id, milestone_id, uri)
    }

    fn contribute(
        env: Env,
        sender: Address,
        amount: i128,
        campaign_id: String,
    ) -> Result<(), Error> {
        contribute(&env, sender, campaign_id, amount)
    }

    fn refund(env: Env, sender: Address, campaign_id: String) -> Result<(), Error> {
        refund(&env, sender, campaign_id)
    }

    fn validate_milestone(
        env: Env,
        campaign_id: String,
        milestone_id: String,
        milestone_sequence: u32,
    ) -> Result<(), Error> {
        validate_milestone(&env, campaign_id, milestone_id, milestone_sequence)
    }

    fn withdraw(env: Env, campaign_id: String) -> Result<(), Error> {
        withdraw(&env, campaign_id)
    }
}

impl Contract {
    // === EVENTS ===
    // The following events are available but implemented in separate modules:
    // - role_added(env: &Env, role: u32, address: Address)
    // - role_removed(env: &Env, role: u32, address: Address)
    // - campaign_added(env: &Env, owner: Address, goal: i128, uri: String, min_donation: i128)
    // - withdraw(env: &Env, owner: &Address, total_raised: i128)
    // - milestone_added(env: &Env, campaign_id: String, milestone_id: String, amount: i128)
    // - proof_added(env: &Env, campaign_id: String, milestone_id: String)
    // - contribution_done(env: &Env, address: Address, amount: i128)
}
