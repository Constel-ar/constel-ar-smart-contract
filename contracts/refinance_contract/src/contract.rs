use soroban_sdk::{contract, contractimpl, Address, Env, String};

use crate::{
    methods::{
        admin::set_admin::set_admin,
        public::{initialize::initialize, transfer::transfer},
        user::user::{get_user, update_transfer_amount},
    },
    storage::{
        structs::campaign::Campaign,
        types::{error::Error, user::User},
    },
};

pub trait ContractTrait {
    fn __constructor(env: Env, admin: Address) -> Result<(), Error>;

    // === CAMPAIGN FUNCTIONS ===
    fn add_campaign(
        env: Env,
        campaign_id: String, // TODO: Usar symbol, address, UUID back
        creator: Address,
        goal: i128,
        min_donation: i128,
    ) -> Result<(), Error>;

    fn get_campaign(env: Env, campaign_id: String) -> Result<Campaign, Error>;

    fn set_admin(env: Env, admin: Address) -> Result<Address, Error>;

    fn transfer(
        env: Env,
        from: Address,
        to: Address,
        token: Address,
        amount: i128,
    ) -> Result<i128, Error>;

    fn get_user(env: Env, address: Address) -> Result<User, Error>;
}

#[contract]
pub struct Contract;

#[contractimpl]
impl ContractTrait for Contract {
    fn __constructor(env: Env, admin: Address) -> Result<(), Error> {
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

    fn get_campaign(env: Env, campaign_id: String) -> Result<Campaign, Error> {
        return Err(Error::NotImplemented); // TODO: U
    }

    // Method from Fede temapltes
    fn set_admin(env: Env, admin: Address) -> Result<Address, Error> {
        set_admin(&env, admin)
    }

    fn transfer(
        env: Env,
        from: Address,
        to: Address,
        token: Address,
        amount: i128,
    ) -> Result<i128, Error> {
        update_transfer_amount(&env, &from)?;
        Ok(transfer(&env, from, to, token, amount))
    }

    fn get_user(env: Env, address: Address) -> Result<User, Error> {
        get_user(&env, &address)
    }
}
