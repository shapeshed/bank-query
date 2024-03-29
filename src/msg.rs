use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Transfer {
        address: String,
        tokens_to_send: Uint128,
        denom: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(cosmwasm_std::BalanceResponse)]
    Balance { address: String, denom: String },
}
