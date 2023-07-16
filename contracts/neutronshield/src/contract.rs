#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError,
    StdResult,
};
use cw2::set_contract_version;

use injective_cosmwasm::{MarketId, Position, SubaccountId};
use injective_math::FPDecimal;
use neutron_sdk::bindings::msg::{IbcFee, MsgSubmitTxResponse, NeutronMsg};
use neutron_sdk::bindings::query::NeutronQuery;
use neutron_sdk::bindings::types::{KVKey, StorageValue};
use neutron_sdk::interchain_queries::types::{KVReconstruct, QueryPayload};
use neutron_sdk::interchain_txs::helpers::get_port_id;
use neutron_sdk::sudo::msg::SudoMsg;
use neutron_sdk::NeutronResult;
use protobuf::Message;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    read_reply_payload, save_sudo_payload, CURRENT_NEW_INJECTIVE_POSITION_ID,
    EXISTING_INTERCHAIN_QUERIES,
};

use self::execute::register_position_query;
use self::query::{get_registered_query, query_interchain_address_contract};
use self::sudo::sudo_open_ack;
use crate::proto_types::register_query::MsgRegisterInterchainQueryResponse;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:counter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const REGISTER_POSITION_QUERY_REPLY_ID: u64 = 1;
pub const SUDO_PAYLOAD_REPLY_ID: u64 = 2;

const DEFAULT_TIMEOUT_SECONDS: u64 = 60 * 60 * 24 * 7 * 2;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
struct OpenAckVersion {
    version: String,
    controller_connection_id: String,
    host_connection_id: String,
    address: String,
    encoding: String,
    tx_type: String,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> NeutronResult<Response<NeutronMsg>> {
    match msg {
        ExecuteMsg::AvoidInjectiveLiquidation {
            market_id,
            subaccount_id,
        } => execute::avoid_injective_liquidation(deps, env, market_id, subaccount_id),
        ExecuteMsg::Register {
            connection_id,
            interchain_account_id,
        } => execute::execute_register_ica(deps, env, connection_id, interchain_account_id),
        ExecuteMsg::RegisterInjectivePositionQuery {
            connection_id,
            market_id,
            subaccount_id,
            update_period,
        } => register_position_query(deps, connection_id, market_id, subaccount_id, update_period),
    }
}

pub fn is_position_close_to_liquidation(
    position: Position,
    oracle_price: FPDecimal,
    maintenance_margin_ratio: FPDecimal,
    min_proximity_to_liquidation: FPDecimal,
) -> bool {
    let position_margin_ratio = position.margin / (oracle_price * position.quantity);
    let proximity_to_liquidation = position_margin_ratio / maintenance_margin_ratio;
    proximity_to_liquidation <= min_proximity_to_liquidation
}

pub mod execute {
    use cosmwasm_std::{CosmosMsg, StdResult, SubMsg};
    use injective_cosmwasm::{MarketId, SubaccountId};
    use neutron_sdk::{bindings::types::ProtobufAny, query::min_ibc_fee::query_min_ibc_fee};

    use crate::state::{save_reply_payload, SudoPayload, INTERCHAIN_ACCOUNTS};

    use super::{
        query::{get_ica, query_position},
        *,
    };

    pub fn avoid_injective_liquidation(
        deps: DepsMut<NeutronQuery>,
        env: Env,
        market_id: MarketId,
        subaccount_id: SubaccountId,
    ) -> NeutronResult<Response<NeutronMsg>> {
        let registered_query_id = EXISTING_INTERCHAIN_QUERIES
            .load(deps.storage, (market_id.as_str(), subaccount_id.as_str()))?;
        let position = query_position(deps.as_ref(), env.to_owned(), registered_query_id)?.position;

        let oracle_price = FPDecimal::must_from_str("14.5"); // TODO read via interchain query
        let maintenance_margin_ratio = FPDecimal::must_from_str("0.02"); // TODO read via interchain query
        let min_proximity_to_liquidation = FPDecimal::must_from_str("1.4"); // TODO allow user to define this value

        if is_position_close_to_liquidation(
            position.0,
            oracle_price,
            maintenance_margin_ratio,
            min_proximity_to_liquidation,
        ) {
            return execute_increase_position_margin_message(
                deps,
                env,
                "injective".to_string(),
                None,
                market_id,
                subaccount_id,
                2000u128.into(), // TODO allow user to define this and/or derive dynamically
            );
        }

        Ok(Response::default())
    }

    pub fn execute_increase_position_margin_message(
        mut deps: DepsMut<NeutronQuery>,
        env: Env,
        interchain_account_id: String,
        timeout: Option<u64>,
        _market_id: MarketId,
        _subaccount_id: SubaccountId,
        _amount: FPDecimal,
    ) -> NeutronResult<Response<NeutronMsg>> {
        // Get the delegator address from the storage & form the Delegate message.
        let ica_response = get_ica(deps.as_ref(), &env, &interchain_account_id)?;

        // TODO fix this
        // let increase_position_margin_msg =
        //     injective_std::types::injective::exchange::v1beta1::MsgIncreasePositionMargin {
        //         market_id: market_id.as_str().to_string(),
        //         amount: amount.to_string(),
        //         sender: ica_response.address,
        //         source_subaccount_id: subaccount_id.to_string(),
        //         destination_subaccount_id: subaccount_id.to_string(),
        //     };

        // let mut buf = Vec::new();
        // buf.reserve(increase_position_margin_msg.encoded_len());

        // if let Err(e) = increase_position_margin_msg.encode(&mut buf) {
        //     return Err(neutron_sdk::NeutronError::Std(StdError::generic_err(format!(
        //         "Encode error: {}",
        //         e
        //     ))));
        // }

        // TODO wrap in authz MsgExec
        let any_msg = ProtobufAny {
            type_url: "/injective.exchange.v1beta1.MsgIncreasePositionMargin".to_string(),
            // value: Binary::from(buf),
            value: Binary::from_base64(
                "eyAiaW5jcmVhc2VfcG9zaXRpb25fbWFyZ2luIjogeyAiYW1vdW50IjogMTAwIH19",
            )?,
        };

        // contract must pay for relaying of acknowledgements
        // See more info here: https://docs.neutron.org/neutron/feerefunder/overview
        let fee = min_ntrn_ibc_fee(query_min_ibc_fee(deps.as_ref())?.min_fee);

        let cosmos_msg = NeutronMsg::submit_tx(
            ica_response.controller_connection_id,
            interchain_account_id.clone(),
            vec![any_msg],
            "".to_string(),
            timeout.unwrap_or(DEFAULT_TIMEOUT_SECONDS),
            fee,
        );

        // We use a submessage here because we need the process message reply to save
        // the outgoing IBC packet identifier for later.
        let submsg = msg_with_sudo_callback(
            deps.branch(),
            cosmos_msg,
            SudoPayload {
                port_id: get_port_id(env.contract.address.as_str(), &interchain_account_id),
                message: "message".to_string(),
            },
        )?;

        Ok(Response::default().add_submessages(vec![submsg]))
    }

    // saves payload to process later to the storage and returns a SubmitTX Cosmos SubMsg with necessary reply id
    pub fn msg_with_sudo_callback<C: Into<CosmosMsg<T>>, T>(
        deps: DepsMut<NeutronQuery>,
        msg: C,
        payload: SudoPayload,
    ) -> StdResult<SubMsg<T>> {
        save_reply_payload(deps.storage, payload)?;
        Ok(SubMsg::reply_on_success(msg, SUDO_PAYLOAD_REPLY_ID))
    }

    pub fn register_position_query(
        deps: DepsMut<NeutronQuery>,
        connection_id: String,
        market_id: MarketId,
        subaccount_id: SubaccountId,
        update_period: u64,
    ) -> NeutronResult<Response<NeutronMsg>> {
        let msg = new_register_position_query_msg(
            connection_id,
            market_id.to_owned(),
            subaccount_id.to_owned(),
            update_period,
        )?;

        CURRENT_NEW_INJECTIVE_POSITION_ID.save(deps.storage, &(market_id, subaccount_id))?;

        let submessage = SubMsg::reply_on_success(msg, REGISTER_POSITION_QUERY_REPLY_ID);
        Ok(Response::new().add_submessage(submessage))
    }

    pub fn execute_register_ica(
        deps: DepsMut<NeutronQuery>,
        env: Env,
        connection_id: String,
        interchain_account_id: String,
    ) -> NeutronResult<Response<NeutronMsg>> {
        let register =
            NeutronMsg::register_interchain_account(connection_id, interchain_account_id.clone());
        let key = get_port_id(env.contract.address.as_str(), &interchain_account_id);

        // we are saving empty data here because we handle response of registering ICA in sudo_open_ack method
        INTERCHAIN_ACCOUNTS.save(deps.storage, key, &None)?;

        Ok(Response::new().add_message(register))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut<NeutronQuery>, env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        REGISTER_POSITION_QUERY_REPLY_ID => {
            let response: MsgRegisterInterchainQueryResponse = Message::parse_from_bytes(
                msg.result
                    .into_result()
                    .expect("Failed to parse reply result")
                    .data
                    .ok_or_else(|| StdError::generic_err("Missing reply data"))?
                    .as_slice(),
            )
            .map_err(|_err| StdError::generic_err("Invalid reply data"))?;

            let (market_id, subaccount_id) =
                CURRENT_NEW_INJECTIVE_POSITION_ID.load(deps.storage)?;

            EXISTING_INTERCHAIN_QUERIES.save(
                deps.storage,
                (market_id.as_str(), subaccount_id.as_str()),
                &response.id,
            )?;
            CURRENT_NEW_INJECTIVE_POSITION_ID.remove(deps.storage);

            Ok(Response::default())
        }
        SUDO_PAYLOAD_REPLY_ID => prepare_sudo_payload(deps, env, msg),
        _ => Err(StdError::generic_err("Invalid reply id")),
    }
}

// prepare_sudo_payload is called from reply handler
// The method is used to extract sequence id and channel from SubmitTxResponse to process sudo payload defined in msg_with_sudo_callback later in Sudo handler.
// Such flow msg_with_sudo_callback() -> reply() -> prepare_sudo_payload() -> sudo() allows you "attach" some payload to your SubmitTx message
// and process this payload when an acknowledgement for the SubmitTx message is received in Sudo handler
pub fn prepare_sudo_payload(
    mut deps: DepsMut<NeutronQuery>,
    _env: Env,
    msg: Reply,
) -> StdResult<Response> {
    let payload = read_reply_payload(deps.storage)?;
    let response: MsgSubmitTxResponse = serde_json_wasm::from_slice(
        msg.result
            .into_result()
            .map_err(StdError::generic_err)?
            .data
            .ok_or_else(|| StdError::generic_err("no result"))?
            .as_slice(),
    )
    .map_err(|e| StdError::generic_err(format!("failed to parse response: {:?}", e)))?;

    deps.api
        .debug(format!("WASMDEBUG: reply msg: {:?}", response).as_str());

    let seq_id = response.sequence_id;
    let channel_id = response.channel;
    save_sudo_payload(deps.branch().storage, channel_id, seq_id, payload)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    msg: SudoMsg,
) -> NeutronResult<Response<NeutronMsg>> {
    match msg {
        SudoMsg::OpenAck {
            port_id,
            channel_id,
            counterparty_channel_id,
            counterparty_version,
        } => sudo_open_ack(
            deps,
            env,
            port_id,
            channel_id,
            counterparty_channel_id,
            counterparty_version,
        ),
        _ => Ok(Response::default()),
    }
}

pub mod sudo {
    use cosmwasm_std::{DepsMut, Env, Response, StdError};
    use neutron_sdk::{
        bindings::{msg::NeutronMsg, query::NeutronQuery},
        NeutronError, NeutronResult,
    };

    use crate::state::INTERCHAIN_ACCOUNTS;

    use super::OpenAckVersion;

    pub fn sudo_open_ack(
        deps: DepsMut<NeutronQuery>,
        _env: Env,
        port_id: String,
        _channel_id: String,
        _counterparty_channel_id: String,
        counterparty_version: String,
    ) -> NeutronResult<Response<NeutronMsg>> {
        // The version variable contains a JSON value with multiple fields,
        // including the generated account address.
        let parsed_version: Result<OpenAckVersion, _> =
            serde_json_wasm::from_str(counterparty_version.as_str());

        // Update the storage record associated with the interchain account.
        if let Ok(parsed_version) = parsed_version {
            INTERCHAIN_ACCOUNTS.save(
                deps.storage,
                port_id,
                &Some((
                    parsed_version.address,
                    parsed_version.controller_connection_id,
                )),
            )?;
            return Ok(Response::default());
        }
        Err(NeutronError::Std(StdError::generic_err(
            "Can't parse counterparty_version",
        )))
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct KVReconstructPosition(Position);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PositionResponse {
    pub position: KVReconstructPosition,
    pub last_submitted_local_height: u64,
}

impl KVReconstruct for KVReconstructPosition {
    fn reconstruct(storage_values: &[StorageValue]) -> NeutronResult<KVReconstructPosition> {
        let position: Position = from_binary(
            &storage_values
                .first()
                .expect("No storage values found")
                .value,
        )?;

        Ok(KVReconstructPosition(position))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InterchainAccountAddressResponse {
    pub address: String,
    pub controller_connection_id: String,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<NeutronQuery>, env: Env, msg: QueryMsg) -> NeutronResult<Binary> {
    match msg {
        QueryMsg::GetRegisteredQuery { query_id } => {
            Ok(to_binary(&get_registered_query(deps, query_id)?)?)
        }
        QueryMsg::InterchainAccountAddressFromContract {
            interchain_account_id,
        } => query_interchain_address_contract(deps, env, interchain_account_id),
    }
}

pub mod query {
    use cosmwasm_std::{CustomQuery, StdError};
    use neutron_sdk::{
        bindings::query::{NeutronQuery, QueryRegisteredQueryResponse},
        interchain_queries::{check_query_type, query_kv_result, types::QueryType},
    };

    use crate::state::INTERCHAIN_ACCOUNTS;

    use super::*;

    /// Queries registered query info
    pub fn get_registered_query(
        deps: Deps<NeutronQuery>,
        interchain_query_id: u64,
    ) -> NeutronResult<QueryRegisteredQueryResponse> {
        let query = NeutronQuery::RegisteredInterchainQuery {
            query_id: interchain_query_id,
        };

        let res: QueryRegisteredQueryResponse = deps.querier.query(&query.into())?;
        Ok(res)
    }

    // returns ICA address from the contract storage. The address was saved in sudo_open_ack method
    pub fn query_interchain_address_contract(
        deps: Deps<NeutronQuery>,
        env: Env,
        interchain_account_id: String,
    ) -> NeutronResult<Binary> {
        Ok(to_binary(&get_ica(deps, &env, &interchain_account_id)?)?)
    }

    pub fn get_ica(
        deps: Deps<impl CustomQuery>,
        env: &Env,
        interchain_account_id: &str,
    ) -> Result<InterchainAccountAddressResponse, StdError> {
        let key = get_port_id(env.contract.address.as_str(), interchain_account_id);

        let (address, controller_connection_id) = INTERCHAIN_ACCOUNTS
            .load(deps.storage, key)?
            .ok_or_else(|| StdError::generic_err("Interchain account is not created yet"))?;

        Ok(InterchainAccountAddressResponse {
            address,
            controller_connection_id,
        })
    }

    /// Returns bank total supply on remote chain for particular denom
    /// * ***registered_query_id*** is an identifier of the corresponding registered interchain query
    pub fn query_position(
        deps: Deps<NeutronQuery>,
        _env: Env,
        registered_query_id: u64,
    ) -> NeutronResult<PositionResponse> {
        let registered_query = get_registered_query(deps, registered_query_id)?;

        check_query_type(registered_query.registered_query.query_type, QueryType::KV)?;

        let position: KVReconstructPosition = query_kv_result(deps, registered_query_id)?;

        Ok(PositionResponse {
            last_submitted_local_height: registered_query
                .registered_query
                .last_submitted_result_local_height,
            position,
        })
    }
}

pub const DERIVATIVE_POSITIONS_PREFIX: u8 = 0x27;
pub const EXCHANGE_STORE_KEY: &str = "exchange";

// https://github.com/OpenDeFiFoundation/injective-core/blob/b094a25889a1270872b3584f2a232cce7a2daccc/injective-chain/modules/exchange/types/key.go#L439-L442C2
pub fn create_subaccount_position_prefix<AddrBytes: AsRef<[u8]>>(
    market_id: AddrBytes,
    subaccount_id: AddrBytes,
) -> NeutronResult<Vec<u8>> {
    let mut prefix: Vec<u8> = vec![DERIVATIVE_POSITIONS_PREFIX];

    prefix.extend_from_slice(market_id.as_ref());
    prefix.extend_from_slice(subaccount_id.as_ref());

    Ok(prefix)
}

/// Creates a message to register an Interchain Query to get position of a subaccount id on Injective
///
/// * **connection_id** is an IBC connection identifier between Neutron and remote chain;
/// * **market_id** market id of the position for which you want to get the position;
/// * **subaccount_id** subaccount id of the position for which you want to get the position;
/// * **denom** denomination of the coin for which you want to get the position;
/// * **update_period** is used to say how often the query must be updated.
pub fn new_register_position_query_msg(
    connection_id: String,
    market_id: MarketId,
    subaccount_id: SubaccountId,
    update_period: u64,
) -> NeutronResult<NeutronMsg> {
    let converted_market_id_bytes: Vec<u8> = market_id.as_str().into();
    let converted_subaccount_id_bytes: Vec<u8> = subaccount_id.as_str().into();

    let balance_key = create_subaccount_position_prefix(
        converted_market_id_bytes,
        converted_subaccount_id_bytes,
    )?;

    let kv_key = KVKey {
        path: EXCHANGE_STORE_KEY.to_string(),
        key: Binary(balance_key),
    };

    NeutronMsg::register_interchain_query(
        QueryPayload::KV(vec![kv_key]),
        connection_id,
        update_period,
    )
}

const FEE_DENOM: &str = "untrn";

pub fn min_ntrn_ibc_fee(fee: IbcFee) -> IbcFee {
    IbcFee {
        recv_fee: fee.recv_fee,
        ack_fee: fee
            .ack_fee
            .into_iter()
            .filter(|a| a.denom == FEE_DENOM)
            .collect(),
        timeout_fee: fee
            .timeout_fee
            .into_iter()
            .filter(|a| a.denom == FEE_DENOM)
            .collect(),
    }
}
