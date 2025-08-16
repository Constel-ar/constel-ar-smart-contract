use soroban_sdk::{Address, Env};

use crate::storage::types::error::Error;

use super::types::storage::DataKey;

pub(crate) fn set_token(env: &Env, token: &Address) {
    let key = DataKey::Token;

    env.storage().instance().set(&key, token);
}

pub(crate) fn get_token(env: &Env) -> Result<Address, Error> {
    let key = DataKey::Token;

    env.storage().instance().get(&key).unwrap()
}
