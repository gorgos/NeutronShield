use cosmwasm_std::{from_binary, to_vec, Binary, StdResult, Storage};
use cw_storage_plus::{Item, Map};
use injective_cosmwasm::{MarketId, SubaccountId};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const INTERCHAIN_ACCOUNTS: Map<String, Option<(String, String)>> =
    Map::new("interchain_accounts");

pub const EXISTING_INTERCHAIN_QUERIES: Map<(&str, &str), u64> =
    Map::new("existing_interchain_queries");

pub const CURRENT_NEW_INJECTIVE_POSITION_ID: Item<(MarketId, SubaccountId)> =
    Item::new("current_new_user");

pub const SUDO_PAYLOAD: Map<(String, u64), Vec<u8>> = Map::new("sudo_payload");

/// SudoPayload is a type that stores information about a transaction that we try to execute
/// on the host chain. This is a type introduced for our convenience.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SudoPayload {
    pub message: String,
    pub port_id: String,
}

pub const REPLY_ID_STORAGE: Item<Vec<u8>> = Item::new("reply_queue_id");

pub fn save_reply_payload(store: &mut dyn Storage, payload: SudoPayload) -> StdResult<()> {
    REPLY_ID_STORAGE.save(store, &to_vec(&payload)?)
}

pub fn read_reply_payload(store: &mut dyn Storage) -> StdResult<SudoPayload> {
    let data = REPLY_ID_STORAGE.load(store)?;
    from_binary(&Binary(data))
}

pub fn save_sudo_payload(
    store: &mut dyn Storage,
    channel_id: String,
    seq_id: u64,
    payload: SudoPayload,
) -> StdResult<()> {
    SUDO_PAYLOAD.save(store, (channel_id, seq_id), &to_vec(&payload)?)
}
