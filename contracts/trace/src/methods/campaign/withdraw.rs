use soroban_sdk::{Env, String};

use crate::{
    events,
    methods::token::token_transfer,
    storage::{
        campaign::{get_campaign, set_campaign},
        types::error::Error,
    },
};

pub fn withdraw(env: &Env, campaign_id: String) -> Result<(), Error> {
    let mut campaign = get_campaign(env, campaign_id.clone())?;

    // Only the campaign owner can withdraw.
    campaign.owner.require_auth();

    let amount = campaign.withdrawable_amount;
    if amount <= 0 {
        return Err(Error::WithdrawalAmountZero);
    }

    // Reset withdrawable amount before transfer for security (re-entrancy).
    campaign.withdrawable_amount = 0;
    set_campaign(env, &campaign_id, &campaign);

    // Transfer funds to the owner.
    token_transfer(
        env,
        &env.current_contract_address(),
        &campaign.owner,
        &amount,
    )?;

    // Emit event
    events::milestone::milestone_withdrawal(env, campaign_id, campaign.current_milestone, amount);

    Ok(())
}
