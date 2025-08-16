use soroban_sdk::{
    token::{self},
    Address, Env,
};

use crate::storage::{token::get_token, types::error::Error};

pub fn token_transfer(env: &Env, from: &Address, to: &Address, amount: &i128) -> Result<(), Error> {
    let token_id = get_token(env)?;
    let token = token::Client::new(env, &token_id);
    token.transfer(from, to, amount);
    Ok(())
}