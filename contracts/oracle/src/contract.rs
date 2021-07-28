use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, HandleResult, InitResponse, Querier,
    QueryRequest, StdResult, Storage, WasmQuery,
};

use crate::state::{read_config, store_config, Config};
use band_protocol::oracle::{QueryMsg as OracleQueryMsg, ReferenceData};
use scrt_prediction::oracle::{ConfigResponse, HandleMsg, InitMsg, PriceData, QueryMsg};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let config = Config {
        band_oracle: deps.api.canonical_address(&msg.band_oracle)?,
        band_oracle_code_hash: msg.band_oracle_code_hash,
        base_symbol: msg.base_symbol,
        quote_symbol: msg.quote_symbol,
    };

    store_config(&mut deps.storage, &config)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    _msg: HandleMsg,
) -> HandleResult {
    Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::QueryLatestPrice {} => to_binary(&query_latest_price(deps)?),
    }
}

fn query_config<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<ConfigResponse> {
    let config: Config = read_config(&deps.storage)?;
    let resp = ConfigResponse {
        band_oracle: deps.api.human_address(&config.band_oracle)?,
        band_oracle_code_hash: config.band_oracle_code_hash,
        base_symbol: config.base_symbol,
        quote_symbol: config.quote_symbol,
    };

    Ok(resp)
}

fn query_latest_price<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<PriceData> {
    let config: Config = read_config(&deps.storage)?;
    let reference_data: ReferenceData =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: deps.api.human_address(&config.band_oracle)?,
            callback_code_hash: config.band_oracle_code_hash,
            msg: to_binary(&OracleQueryMsg::GetReferenceData {
                base_symbol: config.base_symbol,
                quote_symbol: config.quote_symbol,
            })?,
        }))?;

    Ok(PriceData {
        rate: reference_data.rate,
        last_updated: reference_data.last_updated_base,
    })
}
