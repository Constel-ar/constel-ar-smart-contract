use soroban_sdk::{Env, String};

use crate::storage::{structs::proof::Proof, types::error::Error};

use super::types::storage::DataKey;

pub(crate) fn set_proof(env: &Env, milestone_id: String, proof: &Proof) {
    let key = DataKey::Proof(milestone_id);

    env.storage().persistent().set(&key, proof);
}

pub(crate) fn get_proof(env: &Env, milestone_id: String) -> Result<Proof, Error> {
    let key = DataKey::Proof(milestone_id);

    env.storage()
        .persistent()
        .get(&key)
        .ok_or(Error::ProofNotFound)
}

pub(crate) fn has_proof(env: &Env, milestone_id: String) -> bool {
    let key = DataKey::Proof(milestone_id);
    env.storage().persistent().has(&key)
}
