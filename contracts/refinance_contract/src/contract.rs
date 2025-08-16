use soroban_sdk::{contract, contractimpl, Address, Env, Map, String};

use crate::{
    contract_trait::ContractTrait,
    methods::{campaign::add_campaign::add_campaign, public::initialize::initialize},
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
        return Err(Error::NotImplemented);
    }

    fn batch_add_milestones(
        env: Env,
        campaign_id: String,
        milestones: Map<String, i128>,
    ) -> Result<(), Error> {
        return Err(Error::NotImplemented);
    }

    fn add_proof(
        env: Env,
        campaign_id: String,
        milestone_id: String,
        uri: String,
    ) -> Result<(), Error> {
        crate::methods::proofs::proof::add_proof_logic(&env, campaign_id, milestone_id, uri)
    }

    fn contribute(
        env: Env,
        sender: Address,
        amount: i128,
        campaign_id: String,
    ) -> Result<(), Error> {
        return Err(Error::NotImplemented); // TODO: U
    }

    fn refund(env: Env, sender: Address, amount: i128, campaign_id: String) -> Result<(), Error> {
        return Err(Error::NotImplemented); // TODO: U
    }
}

impl Contract {
    // === EVENTS ===
    /**
     *
     fn role_added(env: &Env, role: u32, address: Address)

     fn role_removed(env: &Env, role: u32, address: Address);

     fn campaign_added(env: &Env, owner: Address, goal: i128, uri: String, min_donation: i128);

     fn withdraw(env: &Env, owner: &Address, total_raised: i128);

     fn milestone_added(env: &Env, campaign_id: String, milestone_id: String, amount: i128);

    fn proof_added(env: &Env, campaign_id: String, milestone_id: String);

    fn contribution_done(env: &Env, address: Address, amount: i128);

    fn withdraw(env: Env, sender: Address, amount: i128, campaign_id: String) -> Result<(), Error>; // privada
    */
    fn withdraw(env: Env, sender: Address, amount: i128, campaign_id: String) -> Result<(), Error> {
        return Err(Error::NotImplemented); // TODO: U
    }
}
