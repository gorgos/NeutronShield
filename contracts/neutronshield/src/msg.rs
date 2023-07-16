use cosmwasm_schema::{cw_serde, QueryResponses};
use injective_cosmwasm::{MarketId, SubaccountId};
use neutron_sdk::bindings::query::QueryRegisteredQueryResponse;

use crate::contract::InterchainAccountAddressResponse;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    AvoidInjectiveLiquidation {
        market_id: MarketId,
        subaccount_id: SubaccountId,
    },
    Register {
        connection_id: String,
        interchain_account_id: String,
    },
    RegisterInjectivePositionQuery {
        connection_id: String,
        update_period: u64,
        market_id: MarketId,
        subaccount_id: SubaccountId,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QueryRegisteredQueryResponse)]
    GetRegisteredQuery { query_id: u64 },

    // this query returns ICA from contract store, which saved from acknowledgement
    #[returns(InterchainAccountAddressResponse)]
    InterchainAccountAddressFromContract { interchain_account_id: String },
}
