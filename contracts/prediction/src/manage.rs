use cosmwasm_std::{
    log, Api, Decimal, Env, Extern, HandleResponse, HandleResult, HumanAddr, Querier, StdError,
    Storage,
};

use crate::state::{read_config, store_config, Config};

pub fn update_config<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    owner_addr: Option<HumanAddr>,
    operator_addr: Option<HumanAddr>,
    treasury_addr: Option<HumanAddr>,
    oracle_addr: Option<HumanAddr>,
    fee_rate: Option<Decimal>,
    interval: Option<u64>,
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
    }

    if let Some(fee_rate) = fee_rate {
        config.fee_rate = fee_rate;
    }

    if let Some(interval) = interval {
        config.interval = interval;
    }

    store_config(&mut deps.storage, &config)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![log("action", "update_config"), log("status", "success")],
        data: None,
    })
}
