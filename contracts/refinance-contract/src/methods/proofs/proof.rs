use crate::storage::{
    campaign::get_campaign, proof::set_proof, structs::proof::Proof, types::error::Error,
};
use soroban_sdk::{Env, String};

pub fn add_proof_logic(
    env: &Env,
    campaign_id: String,
    milestone_id: String,
    uri: String,
) -> Result<(), Error> {
    let campaign = get_campaign(env, campaign_id.clone())?;

    campaign.owner.require_auth();

    let proof = Proof {
        milestone_id: milestone_id.clone(),
        uri: uri.clone(),
    };

    set_proof(env, milestone_id.clone(), &proof);

    // Emitir evento
    env.events().publish(
        ("proof_added", campaign_id.clone(), milestone_id),
        uri.clone(),
    );

    Ok(())
}
