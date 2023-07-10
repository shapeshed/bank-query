#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, to_binary};
use cosmwasm_std::{
    BalanceResponse, BankQuery, Binary, Deps, DepsMut, Env, MessageInfo, QueryRequest, Response,
    StdResult, Uint128,
};

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
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
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
        let owner_balance = Uint128::from(1000u128);

        app.sudo(SudoMsg::Bank(BankSudo::Mint {
            to_address: Addr::unchecked("owner").to_string(),
            amount: vec![coin(owner_balance.into(), "PICA".to_string())],
        }))
        .unwrap();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &Empty {},
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let resp: Uint128 = app
            .wrap()
            .query_wasm_smart(
                addr,
                &QueryMsg::Balance {
                    address: Addr::unchecked("owner").to_string(),
                    denom: "PICA".to_string(),
                },
            )
            .unwrap();

        assert_eq!(resp, owner_balance);
    }
}
