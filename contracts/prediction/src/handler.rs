use cosmwasm_std::{
    log, Api, Env, Extern, HandleResponse, HandleResult, HumanAddr, Querier, StdError, Storage,
    Uint128,
};

use crate::state::{
    read_bet, read_round, read_state, store_bet, store_round, Bet, Position, Round, State,
};

pub fn bet<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    user: HumanAddr,
    position: Position,
    amount: Uint128,
) -> HandleResult {
    let state: State = read_state(&deps.storage)?;
    let mut round: Round = read_round(&deps.storage, state.epoch)?;

    if round.start_time > env.block.time {
        return Err(StdError::generic_err("Round not started"));
    }

    if round.lock_time <= env.block.time {
        return Err(StdError::generic_err("Round locked"));
    }

    let user_bet = read_bet(
        &deps.storage,
        state.epoch,
        deps.api.canonical_address(&user)?,
    );

    if user_bet.is_ok() {
        return Err(StdError::generic_err("Already bet"));
    }

    round.total_amount = round.total_amount + amount;

    if position == Position::UP {
        round.up_amount = round.up_amount + amount;
    } else if position == Position::DOWN {
        round.down_amount = round.down_amount + amount;
    }

    store_round(&mut deps.storage, state.epoch, &round)?;

    store_bet(
        &mut deps.storage,
        state.epoch,
        deps.api.canonical_address(&user)?,
        &Bet {
            amount,
            position,
            claimed: false,
        },
    )?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![log("action", "bet"), log("amount", amount)],
        data: None,
    })
}
