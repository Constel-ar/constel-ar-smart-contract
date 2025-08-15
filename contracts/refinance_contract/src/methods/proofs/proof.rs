use soroban_sdk::{Env, String, Address};
use crate::storage::types::error::Error;

// Generar clave de storage para proof
fn proof_storage_key(campaign_id: &String, milestone_id: &String) -> String {
    String::from_str(&format!("proof_{}_{}", campaign_id, milestone_id))
}

pub fn add_proof_logic(
    env: &Env,
    campaign_id: String,
    milestone_id: String,
    uri: String,
) -> Result<(), Error> {
    // Validar campaña
    let campaign_key = String::from_str(&format!("campaign_{}", campaign_id));
    if !env.storage().has(&campaign_key) {
        return Err(Error::CampaignNotFound);
    }

    // Validar milestone
    let milestone_key = String::from_str(&format!("milestone_{}_{}", campaign_id, milestone_id));
    if !env.storage().has(&milestone_key) {
        return Err(Error::MilestoneNotFound);
    }

    // Validar dueño
    let caller = env.invoker();
    let owner_key = String::from_str(&format!("owner_{}", campaign_id));
    let owner: Address = env.storage().get_unchecked(&owner_key).unwrap();
    if caller != owner {
        return Err(Error::Unauthorized);
    }

    // Guardar proof
    let proof_key = proof_storage_key(&campaign_id, &milestone_id);
    env.storage().set(&proof_key, &uri);

    // Emitir evento
    env.events().publish(
        (String::from_str("proof_added"), campaign_id.clone(), milestone_id.clone()),
        uri.clone(),
    );

    Ok(())
}
