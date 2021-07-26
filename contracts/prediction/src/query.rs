use cosmwasm_std::{
    to_binary, Api, Extern, HumanAddr, Querier, QueryRequest, StdResult, Storage, Uint128,
    WasmQuery,
};

use crate::state::{read_bet, read_config, read_round, read_state, Bet, Config, Round, State};
use prediction::oracle::{QueryMsg as OracleQueryMsg, ReferenceData};
use prediction::prediction::ConfigResponse;

pub fn query_config<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<ConfigResponse> {
    let config: Config = read_config(&deps.storage)?;
    let resp = ConfigResponse {
        owner_addr: deps.api.human_address(&config.owner_addr)?,
        operator_addr: deps.api.human_address(&config.operator_addr)?,
        treasury_addr: deps.api.human_address(&config.treasury_addr)?,
        bet_asset: config.bet_asset.to_normal(deps)?,
        oracle_addr: deps.api.human_address(&config.oracle_addr)?,
        oracle_code_hash: config.oracle_code_hash,
        base_symbol: config.base_symbol,
        quote_symbol: config.quote_symbol,
        fee_rate: config.fee_rate,
        interval: config.interval,
        grace_interval: config.grace_interval,
    };

    Ok(resp)
}

pub fn query_state<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<State> {
    let state: State = read_state(&deps.storage)?;
    Ok(state)
}

pub fn query_round<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    epoch: Uint128,
) -> StdResult<Round> {
    let round: Round = read_round(&deps.storage, epoch)?;
    Ok(round)
}

pub fn query_bet<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    epoch: Uint128,
    user: HumanAddr,
) -> StdResult<Bet> {
    let bet: Bet = read_bet(&deps.storage, epoch, deps.api.canonical_address(&user)?)?;
    Ok(bet)
}

pub fn query_price<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    config: Config,
) -> StdResult<ReferenceData> {
    let reference_data: ReferenceData =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: deps.api.human_address(&config.oracle_addr)?,
            callback_code_hash: config.oracle_code_hash,
            msg: to_binary(&OracleQueryMsg::GetReferenceData {
                base_symbol: config.base_symbol,
                quote_symbol: config.quote_symbol,
            })?,
        }))?;

    Ok(reference_data)
}
