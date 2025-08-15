use soroban_sdk::{Address, Env};

use crate::{
    events,
    storage::{
        admin::{
            has_admin, 
            set_admin
        }, 
        token::{
            set_token
        }, 
        types::error::Error
    },
};

pub fn initialize(env: &Env, admin: Address, token: Address) -> Result<(), Error> {
    if has_admin(env) {
        return Err(Error::ContractInitialized);
    }

    set_admin(&env, &admin);
    set_token(&env, &token);
    events::contract::contract_initialized(&env, &admin);

    Ok(())
}
