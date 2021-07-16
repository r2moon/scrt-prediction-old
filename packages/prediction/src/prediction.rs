use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::asset::AssetInfo;
use cosmwasm_std::{Binary, Decimal, HumanAddr, Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    /// Operator address
    pub operator_addr: HumanAddr,
    /// Treasury address
    pub treasury_addr: HumanAddr,
    /// Asset to bet
    pub bet_asset: AssetInfo,
    /// Price oracle address
    pub oracle_addr: HumanAddr,
    /// Fee rate
    pub fee_rate: Decimal,
    /// Interval of each round in seconds
    pub interval: u64,
    /// Grace interval to execute round
    pub grace_interval: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Receive {
        from: HumanAddr,
        msg: Option<Binary>,
        amount: Uint128,
    },
    /// Update configuration
    UpdateConfig {
        owner_addr: Option<HumanAddr>,
        operator_addr: Option<HumanAddr>,
        treasury_addr: Option<HumanAddr>,
        oracle_addr: Option<HumanAddr>,
        fee_rate: Option<Decimal>,
        interval: Option<u64>,
        grace_interval: Option<u64>,
    },
    /// Claim winner reward
    Claim { epoch: Uint128 },
    /// Finish ongoing round, lock betting round and start new round
    ExecuteRound {},
    /// Withdraw performance fee to treasury
    Withdraw {},
    /// Pause
    Pause {},
    /// Start genesis round
    StartGenesisRound {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Query current configuration
    Config {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner_addr: HumanAddr,
    pub operator_addr: HumanAddr,
    pub treasury_addr: HumanAddr,
    pub bet_asset: AssetInfo,
    pub oracle_addr: HumanAddr,
    pub fee_rate: Decimal,
    pub interval: u64,
    pub grace_interval: u64,
}
