use cosmwasm_std::{
    from_binary, to_binary, Api, Binary, Decimal, Env, Extern, HandleResult, HumanAddr,
    InitResponse, Querier, StdError, StdResult, Storage, Uint128,
};

use crate::handler::{bet, claim};
use crate::manage::{execute_round, pause, start_genesis_round, update_config, withdraw};
use crate::msg::Cw20HookMsg;
use crate::query::query_config;
use crate::state::{store_config, store_state, Config, State};
use prediction::prediction::{HandleMsg, InitMsg, QueryMsg};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    if msg.fee_rate > Decimal::one() {
        return Err(StdError::generic_err("Invalid fee rate"));
    }

    if msg.grace_interval > msg.interval {
        return Err(StdError::generic_err("Invalid grace interval"));
    }

    let config = Config {
        owner_addr: deps.api.canonical_address(&env.message.sender)?,
        operator_addr: deps.api.canonical_address(&msg.operator_addr)?,
        treasury_addr: deps.api.canonical_address(&msg.treasury_addr)?,
        bet_asset: msg.bet_asset.to_raw(deps)?,
        oracle_addr: deps.api.canonical_address(&msg.oracle_addr)?,
        oracle_code_hash: msg.oracle_code_hash,
        base_symbol: msg.base_symbol,
        quote_symbol: msg.quote_symbol,
        fee_rate: msg.fee_rate,
        interval: msg.interval,
        grace_interval: msg.grace_interval,
    };

    store_config(&mut deps.storage, &config)?;

    store_state(
        &mut deps.storage,
        &State {
            epoch: Uint128(0),
            total_fee: Uint128(0),
            paused: true,
        },
    )?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> HandleResult {
    match msg {
        HandleMsg::Receive { amount, msg, from } => receive_cw20(deps, env, from, amount, msg),
        HandleMsg::UpdateConfig {
            owner_addr,
            operator_addr,
            treasury_addr,
            oracle_addr,
            oracle_code_hash,
            fee_rate,
            interval,
            grace_interval,
        } => update_config(
            deps,
            env,
            owner_addr,
            operator_addr,
            treasury_addr,
            oracle_addr,
            oracle_code_hash,
            fee_rate,
            interval,
            grace_interval,
        ),
        HandleMsg::Claim { epoch } => claim(deps, env, epoch),
        HandleMsg::Withdraw {} => withdraw(deps, env),
        HandleMsg::ExecuteRound {} => execute_round(deps, env),
        HandleMsg::Pause {} => pause(deps, env),
        HandleMsg::StartGenesisRound {} => start_genesis_round(deps, env),
    }
}

pub fn receive_cw20<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    //todo: figure out if this is "from" or "sender"
    from: HumanAddr,
    amount: Uint128,
    msg: Option<Binary>,
) -> HandleResult {
    if let Some(bin_msg) = msg {
        match from_binary(&bin_msg)? {
            Cw20HookMsg::Bet { position } => bet(deps, env, from, position, amount),
        }
    } else {
        Err(StdError::generic_err("data should be given"))
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
