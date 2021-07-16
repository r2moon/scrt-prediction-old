use cosmwasm_std::{
    log, Api, Decimal, Env, Extern, HandleResponse, HandleResult, HumanAddr, Querier, StdError,
    Storage, Uint128,
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
    } else {
        return Err(StdError::generic_err("Cannot bet DRAW"));
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

pub fn claim<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    epoch: Uint128,
) -> HandleResult {
    let round: Round = read_round(&deps.storage, epoch)?;

    if round.end_time > env.block.time {
        return Err(StdError::generic_err("Round is not ended"));
    }

    if let Some(open_price) = round.open_price {
        if let Some(close_price) = round.close_price {
            let win_position = if close_price > open_price {
                Position::UP
            } else if close_price < open_price {
                Position::DOWN
            } else {
                Position::DRAW
            };

            let mut user_bet = read_bet(
                &deps.storage,
                epoch,
                deps.api.canonical_address(&env.message.sender)?,
            )?;
            let total_win_amount = if win_position == Position::UP {
                round.up_amount
            } else if win_position == Position::DOWN {
                round.down_amount
            } else {
                Uint128(0)
            };

            if user_bet.claimed {
                return Err(StdError::generic_err("Already claimed"));
            }
            if user_bet.position == win_position || win_position == Position::DRAW {
                user_bet.claimed = true;
                store_bet(
                    &mut deps.storage,
                    epoch,
                    deps.api.canonical_address(&env.message.sender)?,
                    &user_bet,
                )?;

                let claim_amount = if win_position == Position::DRAW {
                    user_bet.amount
                } else {
                    round.reward_amount * Decimal::from_ratio(user_bet.amount, total_win_amount)
                };

                Ok(HandleResponse {
                    messages: vec![],
                    log: vec![
                        log("action", "claim"),
                        log("epoch", epoch),
                        log("amount", claim_amount),
                    ],
                    data: None,
                })
            } else {
                return Err(StdError::generic_err("You lose"));
            }
        } else {
            return Err(StdError::generic_err("Round is not closed"));
        }
    } else {
        return Err(StdError::generic_err("Round is not opened"));
    }
}
