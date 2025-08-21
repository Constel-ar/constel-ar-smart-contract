use soroban_sdk::{Env, String};

use crate::storage::{structs::proof::Proof, types::error::Error};

use super::types::storage::DataKey;

pub(crate) fn set_proof(env: &Env, milestone_id: String, proof: &Proof) {
    let key = DataKey::Proof(milestone_id);

    env.storage().instance().set(&key, proof);
}

pub(crate) fn get_proof(env: &Env, milestone_id: String) -> Result<Proof, Error> {
    let key = DataKey::Proof(milestone_id);

    env.storage()
        .instance()
        .get(&key)
        .ok_or(Error::ProofNotFound)
}
