use cosmwasm_std::testing::{mock_dependencies, mock_env, MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{from_binary, Decimal, Extern, HumanAddr, StdError, Uint128};

use scrt_prediction::{
    asset::AssetInfo,
    prediction::{ConfigResponse, HandleMsg, InitMsg, QueryMsg, State},
};

use crate::contract::{handle, init, query};

#[test]
fn test_init_failed_if_fee_rate_is_greater_than_100() {
    let mut deps = mock_dependencies(20, &[]);

    let msg = InitMsg {
        operator_addr: HumanAddr::from("operator_addr"),
        treasury_addr: HumanAddr::from("treasury_addr"),
        bet_asset: AssetInfo::NativeToken {
            denom: "sscrt".to_string(),
        },
        oracle_addr: HumanAddr::from("oracle_addr"),
        oracle_code_hash: String::from("oracle_code_hash"),
        fee_rate: Decimal::percent(101),
        interval: 18000,
        grace_interval: 18000,
    };

    let env = mock_env("addr", &[]);

    let res = init(&mut deps, env, msg).unwrap_err();
    assert_eq!(StdError::generic_err("Invalid fee rate"), res);
}

#[test]
fn test_init_failed_if_grace_interval_is_greater_than_interval() {
    let mut deps = mock_dependencies(20, &[]);

    let msg = InitMsg {
        operator_addr: HumanAddr::from("operator_addr"),
        treasury_addr: HumanAddr::from("treasury_addr"),
        bet_asset: AssetInfo::NativeToken {
            denom: "sscrt".to_string(),
        },
        oracle_addr: HumanAddr::from("oracle_addr"),
        oracle_code_hash: String::from("oracle_code_hash"),
        fee_rate: Decimal::percent(3),
        interval: 18000,
        grace_interval: 18001,
    };

    let env = mock_env("addr", &[]);

    let res = init(&mut deps, env, msg).unwrap_err();
    assert_eq!(StdError::generic_err("Invalid grace interval"), res);
}

#[test]
fn test_init() {
    let mut deps = mock_dependencies(20, &[]);

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

    init(&mut deps, env, msg).unwrap();

    let res = query(&deps, QueryMsg::Config {}).unwrap();
    let config: ConfigResponse = from_binary(&res).unwrap();
    assert_eq!(
        ConfigResponse {
            owner_addr: HumanAddr::from("addr"),
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
        },
        config
    );

    let res = query(&deps, QueryMsg::State {}).unwrap();
    let state: State = from_binary(&res).unwrap();
    assert_eq!(
        State {
            epoch: Uint128::zero(),
            total_fee: Uint128::zero(),
            paused: true,
        },
        state
    );
}
