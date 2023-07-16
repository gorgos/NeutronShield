/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.31.6.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

export interface InstantiateMsg {}
export type ExecuteMsg = {
  avoid_injective_liquidation: {
    market_id: MarketId;
    subaccount_id: SubaccountId;
  };
} | {
  register: {
    connection_id: string;
    interchain_account_id: string;
  };
} | {
  register_injective_position_query: {
    connection_id: string;
    market_id: MarketId;
    subaccount_id: SubaccountId;
    update_period: number;
  };
};
export type MarketId = string;
export type SubaccountId = string;
export type QueryMsg = {
  get_registered_query: {
    query_id: number;
  };
} | {
  interchain_account_address_from_contract: {
    interchain_account_id: string;
  };
};
export type Uint128 = string;
export type Binary = string;
export type QueryType = "kv" | "tx";
export interface QueryRegisteredQueryResponse {
  registered_query: RegisteredQuery;
  [k: string]: unknown;
}
export interface RegisteredQuery {
  connection_id: string;
  deposit?: Coin[];
  id: number;
  keys: KVKey[];
  last_submitted_result_local_height?: number;
  last_submitted_result_remote_height?: Height;
  owner: string;
  query_type: QueryType;
  registered_at_height?: number;
  submit_timeout?: number;
  transactions_filter: string;
  update_period: number;
  [k: string]: unknown;
}
export interface Coin {
  amount: Uint128;
  denom: string;
  [k: string]: unknown;
}
export interface KVKey {
  key: Binary;
  path: string;
  [k: string]: unknown;
}
export interface Height {
  revision_height?: number;
  revision_number?: number;
  [k: string]: unknown;
}
export interface InterchainAccountAddressResponse {
  address: string;
  controller_connection_id: string;
  [k: string]: unknown;
}