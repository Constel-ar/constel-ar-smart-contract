use soroban_sdk::{contract, contractimpl, Address, Env, Map, String};

use crate::{
    methods::{
        public::{
            initialize::initialize},
        },
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

#[contract]
pub struct Contract;

#[contractimpl]
impl ContractTrait for Contract {
    fn __constructor(env: Env, admin: Address, token: Address) -> Result<(), Error> {
        initialize(&env, admin)
    }

    fn add_campaign(
        env: Env,
        campaign_id: String, // TODO: Usar symbol, address, UUID back
        creator: Address,
        goal: i128,
        min_donation: i128,
    ) -> Result<(), Error> {
        return Err(Error::NotImplemented);
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
        milestone_id: String,
        uri: String,
    ) -> Result<(), Error> {
        return Err(Error::NotImplemented); // TODO: U
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
