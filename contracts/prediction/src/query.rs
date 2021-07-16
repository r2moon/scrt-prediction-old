use cosmwasm_std::{Api, Extern, Querier, StdResult, Storage};

use crate::state::{read_config, Config};
use prediction::prediction::ConfigResponse;

pub fn query_config<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<ConfigResponse> {
    let state: Config = read_config(&deps.storage)?;
    let resp = ConfigResponse {
        owner_addr: deps.api.human_address(&state.owner_addr)?,
        operator_addr: deps.api.human_address(&state.operator_addr)?,
        treasury_addr: deps.api.human_address(&state.treasury_addr)?,
        bet_asset: state.bet_asset.to_normal(deps)?,
        oracle_addr: deps.api.human_address(&state.oracle_addr)?,
        fee_rate: state.fee_rate,
        interval: state.interval,
        grace_interval: state.grace_interval,
    };

    Ok(resp)
}
