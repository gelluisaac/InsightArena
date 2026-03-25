use soroban_sdk::{token, Address, Env};

use crate::config;
use crate::errors::InsightArenaError;

/// Transfer `amount` stroops from `predictor` into the contract's escrow.
///
/// The contract address becomes the custodian of the staked XLM; funds are held
/// until the market is resolved (payout) or cancelled (refund).
///
/// # Errors
/// Propagates any error returned by [`config::get_config`].  Token transfer
/// panics are handled by the Soroban runtime and surface as contract failures.
pub fn lock_stake(env: &Env, predictor: &Address, amount: i128) -> Result<(), InsightArenaError> {
    let cfg = config::get_config(env)?;
    token::Client::new(env, &cfg.xlm_token).transfer(
        predictor,
        &env.current_contract_address(),
        &amount,
    );
    Ok(())
}

/// Transfer `amount` stroops from the contract's own escrow balance to `recipient`.
///
/// The contract address is the implicit custodian of all staked XLM; when a
/// market is cancelled every predictor's stake is returned here.
///
/// # Errors
/// Propagates any error returned by [`config::get_config`].  Token transfer
/// panics are handled by the Soroban runtime and surface as contract failures.
pub fn refund(env: &Env, recipient: &Address, amount: i128) -> Result<(), InsightArenaError> {
    let cfg = config::get_config(env)?;
    token::Client::new(env, &cfg.xlm_token).transfer(
        &env.current_contract_address(),
        recipient,
        &amount,
    );
    Ok(())
}

/// Release a winner payout from contract escrow to `predictor`.
///
/// This is semantically distinct from `refund` (used for market cancellation),
/// but uses the same escrow transfer path from contract balance to recipient.
pub fn release_payout(
    env: &Env,
    predictor: &Address,
    amount: i128,
) -> Result<(), InsightArenaError> {
    let cfg = config::get_config(env)?;
    token::Client::new(env, &cfg.xlm_token).transfer(
        &env.current_contract_address(),
        predictor,
        &amount,
    );
    Ok(())
}
