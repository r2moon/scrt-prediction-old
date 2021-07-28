use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{HumanAddr, Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub band_oracle: HumanAddr,
    /// Price oracle code hash
    pub band_oracle_code_hash: String,
    /// Base symbol for price
    pub base_symbol: String,
    /// Quote symbol for price
    pub quote_symbol: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Query current configuration
    Config {},
    /// Query latest price
    QueryLatestPrice {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub band_oracle: HumanAddr,
    pub band_oracle_code_hash: String,
    pub base_symbol: String,
    pub quote_symbol: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceData {
    pub rate: Uint128,
    pub last_updated: u64,
}
