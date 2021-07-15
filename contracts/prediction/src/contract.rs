use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResult, InitResponse, Querier, StdResult, Storage,
};

use crate::manage::update_config;
use crate::query::query_config;
use crate::state::{store_config, Config};
use prediction::prediction::{HandleMsg, InitMsg, QueryMsg};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let config = Config {
        owner_addr: deps.api.canonical_address(&env.message.sender)?,
        operator_addr: deps.api.canonical_address(&msg.operator_addr)?,
        treasury_addr: deps.api.canonical_address(&msg.treasury_addr)?,
        fee_rate: msg.fee_rate,
        interval: msg.interval,
    };

    store_config(&mut deps.storage, &config)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> HandleResult {
    match msg {
        HandleMsg::UpdateConfig {
            owner_addr,
            operator_addr,
            treasury_addr,
            fee_rate,
            interval,
        } => update_config(
            deps,
            env,
            owner_addr,
            operator_addr,
            treasury_addr,
            fee_rate,
            interval,
        ),
    }
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}
