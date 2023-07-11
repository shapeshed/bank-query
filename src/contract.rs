use cosmwasm_std::{
    coin, BalanceResponse, BankMsg, BankQuery, Binary, Deps, DepsMut, Env, MessageInfo,
    QueryRequest, Response, StdResult, Uint128,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, to_binary};

// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:bank-query";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Transfer {
            address,
            tokens_to_send,
            denom,
        } => send_tokens(address, tokens_to_send, denom),
    }
}

pub fn send_tokens(
    to_address: String,
    tokens_to_send: Uint128,
    denom: String,
) -> Result<Response, ContractError> {
    let amount = vec![coin(tokens_to_send.into(), denom)];
    Ok(Response::new().add_message(BankMsg::Send {
        to_address: to_address.clone().into(),
        amount,
    }))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { address, denom } => to_binary(&query_balance(deps, address, denom)?),
    }
}

pub fn query_balance(deps: Deps, address: String, denom: String) -> StdResult<Uint128> {
    let validated_address = deps.api.addr_validate(&address).unwrap();
    let balance: BalanceResponse = deps.querier.query(&QueryRequest::Bank(BankQuery::Balance {
        address: validated_address.to_string(),
        denom,
    }))?;
    Ok(balance.amount.amount)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{coin, Addr, Empty};
    use cw_multi_test::{App, BankSudo, ContractWrapper, Executor, SudoMsg};

    use super::*;

    #[test]
    fn query_balance() {
        let mut app = App::default();
        let deployer = Addr::unchecked("deployer");
        let owner_balance = Uint128::from(1000u128);

        app.sudo(SudoMsg::Bank(BankSudo::Mint {
            to_address: deployer.to_string(),
            amount: vec![coin(owner_balance.into(), "PICA".to_string())],
        }))
        .unwrap();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(code_id, deployer.clone(), &Empty {}, &[], "Contract", None)
            .unwrap();

        let resp: Uint128 = app
            .wrap()
            .query_wasm_smart(
                addr,
                &QueryMsg::Balance {
                    address: deployer.to_string(),
                    denom: "PICA".to_string(),
                },
            )
            .unwrap();

        assert_eq!(resp, owner_balance);
    }

    #[test]
    fn transfer() {
        let mut app = App::default();
        let deployer = Addr::unchecked("deployer");
        let alice = Addr::unchecked("alice");
        let owner_balance = Uint128::from(1000u128);
        let tokens_to_send = Uint128::from(100u128);

        // Mint tokens to the deployer
        app.sudo(SudoMsg::Bank(BankSudo::Mint {
            to_address: deployer.to_string(),
            amount: vec![coin(owner_balance.into(), "PICA".to_string())],
        }))
        .unwrap();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(code_id, deployer.clone(), &Empty {}, &[], "Contract", None)
            .unwrap();

        app.execute_contract(
            deployer.clone(),
            addr.clone(),
            &ExecuteMsg::Transfer {
                address: alice.to_string(),
                tokens_to_send,
                denom: "PICA".to_string(),
            },
            &[],
        )
        .unwrap_err();

        let alice_balance: Uint128 = app
            .wrap()
            .query_wasm_smart(
                addr,
                &QueryMsg::Balance {
                    address: alice.to_string(),
                    denom: "PICA".to_string(),
                },
            )
            .unwrap();

        assert_eq!(tokens_to_send, alice_balance);
    }
}
