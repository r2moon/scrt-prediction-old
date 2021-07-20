use cosmwasm_std::{
    log, Api, Decimal, Env, Extern, HandleResponse, HandleResult, HumanAddr, Querier, StdError,
    Storage, Uint128,
};

use crate::state::{
    read_config, read_round, read_state, store_config, store_round, store_state, Config, Round,
    State,
};

pub fn update_config<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    owner_addr: Option<HumanAddr>,
    operator_addr: Option<HumanAddr>,
    treasury_addr: Option<HumanAddr>,
    oracle_addr: Option<HumanAddr>,
    oracle_code_hash: Option<String>,
    fee_rate: Option<Decimal>,
    interval: Option<u64>,
    grace_interval: Option<u64>,
) -> HandleResult {
    let mut config: Config = read_config(&deps.storage)?;

    // permission check
    if deps.api.canonical_address(&env.message.sender)? != config.owner_addr {
        return Err(StdError::unauthorized());
    }

    if let Some(owner_addr) = owner_addr {
        config.owner_addr = deps.api.canonical_address(&owner_addr)?;
    }

    if let Some(operator_addr) = operator_addr {
        config.operator_addr = deps.api.canonical_address(&operator_addr)?;
    }

    if let Some(treasury_addr) = treasury_addr {
        config.treasury_addr = deps.api.canonical_address(&treasury_addr)?;
    }

    if let Some(oracle_addr) = oracle_addr {
        config.oracle_addr = deps.api.canonical_address(&oracle_addr)?;
        config.oracle_code_hash = oracle_code_hash.unwrap();
    }

    if let Some(fee_rate) = fee_rate {
        if fee_rate > Decimal::one() {
            return Err(StdError::generic_err("Invalid fee rate"));
        }
        config.fee_rate = fee_rate;
    }

    if let Some(interval) = interval {
        config.interval = interval;
    }

    if let Some(grace_interval) = grace_interval {
        if grace_interval > config.interval {
            return Err(StdError::generic_err("Invalid grace interval"));
        }
        config.grace_interval = grace_interval;
    }

    store_config(&mut deps.storage, &config)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![log("action", "update_config"), log("status", "success")],
        data: None,
    })
}

pub fn execute_round<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> HandleResult {
    let config: Config = read_config(&deps.storage)?;

    // permission check
    if deps.api.canonical_address(&env.message.sender)? != config.operator_addr {
        return Err(StdError::unauthorized());
    }

    let mut state: State = read_state(&deps.storage)?;
    if state.paused {
        return Err(StdError::generic_err("Paused"));
    }
    let progressing_epoch = (state.epoch - Uint128(1))?;
    let betting_epoch = state.epoch;
    let mut round: Round = read_round(&deps.storage, progressing_epoch)?;

    if round.expired(env.clone(), config.grace_interval) {
        state.paused = true;
        store_state(&mut deps.storage, &state)?;

        return Ok(HandleResponse {
            messages: vec![],
            log: vec![
                log("action", "execute"),
                log("epoch", progressing_epoch),
                log("status", "expired"),
            ],
            data: None,
        });
    }

    if !round.executable(env.clone(), config.grace_interval) {
        return Err(StdError::generic_err("Cannot execute"));
    }

    // TODO fetch price from band protocol
    let close_price = Uint128(100);
    if let Some(open_price) = round.open_price {
        round.close_price = Some(close_price);

        if close_price != open_price {
            let mut fee = round.total_amount * config.fee_rate;
            round.reward_amount = (round.total_amount - fee)?;

            if close_price > open_price {
                if round.reward_amount < round.up_amount {
                    round.reward_amount = round.total_amount;
                    fee = Uint128(0);
                }
            } else {
                if round.reward_amount < round.down_amount {
                    round.reward_amount = round.total_amount;
                    fee = Uint128(0);
                }
            }

            state.total_fee = state.total_fee + fee;
        }

        // Store result of round
        store_round(&mut deps.storage, progressing_epoch, &round)?;

        let mut betting_round: Round = read_round(&deps.storage, betting_epoch)?;
        betting_round.open_price = Some(close_price);

        // Lock betting round
        store_round(&mut deps.storage, betting_epoch, &betting_round)?;

        // Increase epoch
        state.epoch = state.epoch + Uint128(1);
        store_state(&mut deps.storage, &state)?;

        let new_round = Round {
            start_time: env.block.time,
            lock_time: env.block.time + config.interval,
            end_time: env.block.time + config.interval * 2,
            open_price: None,
            close_price: None,
            total_amount: Uint128(0),
            reward_amount: Uint128(0),
            up_amount: Uint128(0),
            down_amount: Uint128(0),
            is_genesis: false,
        };

        // Start new round
        store_round(&mut deps.storage, state.epoch, &new_round)?;

        Ok(HandleResponse {
            messages: vec![],
            log: vec![
                log("action", "finish"),
                log("epoch", progressing_epoch),
                log("close_price", close_price),
            ],
            data: None,
        })
    } else {
        return Err(StdError::generic_err("Round is not opened"));
    }
}

pub fn withdraw<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> HandleResult {
    let config: Config = read_config(&deps.storage)?;

    // permission check
    if deps.api.canonical_address(&env.message.sender)? != config.owner_addr {
        return Err(StdError::unauthorized());
    }

    let mut state: State = read_state(&deps.storage)?;

    let total_fee = state.total_fee;
    if total_fee > Uint128(0) {
        // TODO withdraw
    } else {
        return Err(StdError::generic_err("No stacked fee"));
    }
    state.total_fee = Uint128(0);

    store_state(&mut deps.storage, &state)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![log("action", "withdraw"), log("amount", total_fee)],
        data: None,
    })
}

pub fn pause<S: Storage, A: Api, Q: Querier>(deps: &mut Extern<S, A, Q>, env: Env) -> HandleResult {
    let config: Config = read_config(&deps.storage)?;

    // permission check
    if deps.api.canonical_address(&env.message.sender)? != config.owner_addr {
        return Err(StdError::unauthorized());
    }

    let mut state: State = read_state(&deps.storage)?;
    if state.paused {
        return Err(StdError::generic_err("Paused"));
    }

    state.paused = true;

    store_state(&mut deps.storage, &state)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![log("action", "pause")],
        data: None,
    })
}

pub fn start_genesis_round<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> HandleResult {
    let config: Config = read_config(&deps.storage)?;

    // permission check
    if deps.api.canonical_address(&env.message.sender)? != config.owner_addr {
        return Err(StdError::unauthorized());
    }

    let mut state: State = read_state(&deps.storage)?;
    if !state.paused {
        return Err(StdError::generic_err("Running now"));
    }

    let epoch = state.epoch + Uint128(1);

    let open_price = Uint128(0);
    // Start genesis round
    store_round(
        &mut deps.storage,
        epoch,
        &Round {
            start_time: env.block.time - config.interval,
            lock_time: env.block.time,
            end_time: env.block.time + config.interval,
            open_price: Some(open_price),
            close_price: None,
            total_amount: Uint128(0),
            reward_amount: Uint128(0),
            up_amount: Uint128(0),
            down_amount: Uint128(0),
            is_genesis: true,
        },
    )?;

    store_round(
        &mut deps.storage,
        epoch + Uint128(1),
        &Round {
            start_time: env.block.time,
            lock_time: env.block.time + config.interval,
            end_time: env.block.time + config.interval * 2,
            open_price: None,
            close_price: None,
            total_amount: Uint128(0),
            reward_amount: Uint128(0),
            up_amount: Uint128(0),
            down_amount: Uint128(0),
            is_genesis: false,
        },
    )?;

    state.paused = false;
    state.epoch = state.epoch + Uint128(2);
    store_state(&mut deps.storage, &state)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![log("action", "pause")],
        data: None,
    })
}
