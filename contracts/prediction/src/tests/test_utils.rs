use cosmwasm_std::testing::{mock_env, MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{Decimal, Extern, HumanAddr};

use scrt_prediction::{asset::AssetInfo, prediction::InitMsg};

use crate::contract::init;

pub fn init_prediction(deps: &mut Extern<MockStorage, MockApi, MockQuerier>) {
    let msg = InitMsg {
        operator_addr: HumanAddr::from("operator_addr"),
        treasury_addr: HumanAddr::from("treasury_addr"),
        bet_asset: AssetInfo::NativeToken {
            denom: "sscrt".to_string(),
        },
        oracle_addr: HumanAddr::from("oracle_addr"),
        oracle_code_hash: String::from("oracle_code_hash"),
        fee_rate: Decimal::percent(5),
        interval: 18000,
        grace_interval: 18000,
    };

    let env = mock_env("addr", &[]);

    let _res = init(deps, env, msg).unwrap();
}
