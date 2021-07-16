use cosmwasm_std::{CanonicalAddr, Decimal, StdResult, Storage, Uint128};
use cosmwasm_storage::{Bucket, ReadonlyBucket, ReadonlySingleton, Singleton};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use prediction::asset::AssetInfoRaw;

static KEY_CONFIG: &[u8] = b"config";
static KEY_STATE: &[u8] = b"state";
static PREFIX_ROUND: &[u8] = b"round";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner_addr: CanonicalAddr,
    pub operator_addr: CanonicalAddr,
    pub treasury_addr: CanonicalAddr,
    pub bet_asset: AssetInfoRaw,
    pub oracle_addr: CanonicalAddr,
    pub fee_rate: Decimal,
    pub interval: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub epoch: Uint128,
    pub total_fee: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Round {
    pub start_time: u64,
    pub lock_time: u64,
    pub end_time: u64,
    pub open_price: Option<Uint128>,
    pub close_price: Option<Uint128>,
    pub total_amount: Uint128,
    pub reward_amount: Uint128,
    pub up_amount: Uint128,
    pub down_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Bet {
    pub amount: Uint128,
    pub position: Position,
    pub claimed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Position {
    UP,
    DOWN,
    DRAW,
}

pub fn store_config<S: Storage>(storage: &mut S, data: &Config) -> StdResult<()> {
    Singleton::new(storage, KEY_CONFIG).save(data)
}
pub fn read_config<S: Storage>(storage: &S) -> StdResult<Config> {
    ReadonlySingleton::new(storage, KEY_CONFIG).load()
}

pub fn store_state<S: Storage>(storage: &mut S, data: &State) -> StdResult<()> {
    Singleton::new(storage, KEY_STATE).save(data)
}
pub fn read_state<S: Storage>(storage: &S) -> StdResult<State> {
    ReadonlySingleton::new(storage, KEY_STATE).load()
}

pub fn store_round<S: Storage>(storage: &mut S, epoch: Uint128, data: &Round) -> StdResult<()> {
    Bucket::new(PREFIX_ROUND, storage).save(&epoch.u128().to_be_bytes(), data)
}
pub fn read_round<S: Storage>(storage: &S, epoch: Uint128) -> StdResult<Round> {
    ReadonlyBucket::new(PREFIX_ROUND, storage).load(&epoch.u128().to_be_bytes())
}

pub fn store_bet<S: Storage>(
    storage: &mut S,
    epoch: Uint128,
    user: CanonicalAddr,
    data: &Bet,
) -> StdResult<()> {
    Bucket::new(PREFIX_ROUND, storage).save(
        &[user.as_slice(), &epoch.u128().to_be_bytes()].concat(),
        data,
    )
}

pub fn read_bet<S: Storage>(storage: &S, epoch: Uint128, user: CanonicalAddr) -> StdResult<Bet> {
    ReadonlyBucket::new(PREFIX_ROUND, storage)
        .load(&[user.as_slice(), &epoch.u128().to_be_bytes()].concat())
}
