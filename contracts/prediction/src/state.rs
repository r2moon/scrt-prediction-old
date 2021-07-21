use cosmwasm_std::{CanonicalAddr, Decimal, Env, StdResult, Storage, Uint128};
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
    pub oracle_code_hash: String,
    pub base_symbol: String,
    pub quote_symbol: String,
    pub fee_rate: Decimal,
    pub interval: u64,
    pub grace_interval: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub epoch: Uint128,
    pub total_fee: Uint128,
    pub paused: bool,
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
    pub is_genesis: bool,
}

impl Round {
    pub fn bettable(&self, env: Env) -> bool {
        !self.is_genesis
            && env.block.time >= self.start_time
            && env.block.time <= self.lock_time
            && self.open_price.is_none()
            && self.close_price.is_none()
    }

    pub fn claimable(&self, env: Env) -> bool {
        env.block.time >= self.end_time
            && self.open_price.is_some()
            && self.close_price.is_some()
            && Some(self.open_price) != Some(self.close_price)
    }

    pub fn refundable(&self, env: Env, grace_interval: u64) -> bool {
        (env.block.time >= self.end_time
            && self.open_price.is_some()
            && self.close_price.is_some()
            && Some(self.open_price) == Some(self.close_price))
            || (self.close_price.is_none() && env.block.time > self.end_time + grace_interval)
    }

    pub fn claimable_amount(&self, env: Env, user_bet: Bet, grace_interval: u64) -> Uint128 {
        if self.claimable(env.clone()) {
            let win_bet_amount = if Some(self.close_price) > Some(self.open_price)
                && user_bet.position == Position::UP
            {
                self.up_amount
            } else if user_bet.position == Position::DOWN {
                self.down_amount
            } else {
                Uint128(0)
            };

            return self.reward_amount * Decimal::from_ratio(user_bet.amount, win_bet_amount);
        }
        if self.refundable(env, grace_interval) {
            return user_bet.amount;
        }
        Uint128(0)
    }

    pub fn executable(&self, env: Env, grace_interval: u64) -> bool {
        env.block.time >= self.end_time
            && env.block.time <= self.end_time + grace_interval
            && self.open_price.is_some()
            && self.close_price.is_none()
    }

    pub fn expired(&self, env: Env, grace_interval: u64) -> bool {
        env.block.time > self.end_time + grace_interval && self.close_price.is_none()
    }

    // pub fn win_position(&self) -> Position {
    //     if Some(self.open_price) > Some(self.close_price) {
    //         Position::DOWN
    //     } else if Some(self.open_price) > Some(self.close_price) {
    //         Position::UP
    //     } else {
    //         Position::DRAW
    //     }
    // }
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
