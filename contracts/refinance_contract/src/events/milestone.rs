use soroban_sdk::{symbol_short, Env, String};

/// Event emitted when a new milestone is created for a campaign
pub(crate) fn milestone_created(
    env: &Env,
    campaign_id: String,
    sequence: u32,
    target_amount: i128,
) {
    env.events().publish(
        (symbol_short!("milestone"), symbol_short!("created")),
        (campaign_id, sequence, target_amount),
    );
}

/// Event emitted when a milestone is completed and validated with proof
pub(crate) fn milestone_completed(
    env: &Env,
    campaign_id: String,
    milestone_sequence: u32,
    proof_id: String,
) {
    env.events().publish(
        (symbol_short!("milestone"), symbol_short!("completed")),
        (campaign_id, milestone_sequence, proof_id),
    );
}

/// Event emitted when funds are withdrawn for a completed milestone
pub(crate) fn milestone_withdrawal(
    env: &Env,
    campaign_id: String,
    milestone_sequence: u32,
    amount: i128,
) {
    env.events().publish(
        (symbol_short!("milestone"), symbol_short!("withdraw")),
        (campaign_id, milestone_sequence, amount),
    );
}
