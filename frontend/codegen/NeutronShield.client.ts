/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.31.6.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import { InstantiateMsg, ExecuteMsg, MarketId, SubaccountId, QueryMsg, Uint128, Binary, QueryType, QueryRegisteredQueryResponse, RegisteredQuery, Coin, KVKey, Height, InterchainAccountAddressResponse } from "./NeutronShield.types";
export interface NeutronShieldReadOnlyInterface {
  contractAddress: string;
  getRegisteredQuery: ({
    queryId
  }: {
    queryId: number;
  }) => Promise<QueryRegisteredQueryResponse>;
  interchainAccountAddressFromContract: ({
    interchainAccountId
  }: {
    interchainAccountId: string;
  }) => Promise<InterchainAccountAddressResponse>;
}
export class NeutronShieldQueryClient implements NeutronShieldReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.getRegisteredQuery = this.getRegisteredQuery.bind(this);
    this.interchainAccountAddressFromContract = this.interchainAccountAddressFromContract.bind(this);
  }

  getRegisteredQuery = async ({
    queryId
  }: {
    queryId: number;
  }): Promise<QueryRegisteredQueryResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_registered_query: {
        query_id: queryId
      }
    });
  };
  interchainAccountAddressFromContract = async ({
    interchainAccountId
  }: {
    interchainAccountId: string;
  }): Promise<InterchainAccountAddressResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      interchain_account_address_from_contract: {
        interchain_account_id: interchainAccountId
      }
    });
  };
}
export interface NeutronShieldInterface extends NeutronShieldReadOnlyInterface {
  contractAddress: string;
  sender: string;
  avoidInjectiveLiquidation: ({
    marketId,
    subaccountId
  }: {
    marketId: MarketId;
    subaccountId: SubaccountId;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  register: ({
    connectionId,
    interchainAccountId
  }: {
    connectionId: string;
    interchainAccountId: string;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  registerInjectivePositionQuery: ({
    connectionId,
    marketId,
    subaccountId,
    updatePeriod
  }: {
    connectionId: string;
    marketId: MarketId;
    subaccountId: SubaccountId;
    updatePeriod: number;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
}
export class NeutronShieldClient extends NeutronShieldQueryClient implements NeutronShieldInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.avoidInjectiveLiquidation = this.avoidInjectiveLiquidation.bind(this);
    this.register = this.register.bind(this);
    this.registerInjectivePositionQuery = this.registerInjectivePositionQuery.bind(this);
  }

  avoidInjectiveLiquidation = async ({
    marketId,
    subaccountId
  }: {
    marketId: MarketId;
    subaccountId: SubaccountId;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      avoid_injective_liquidation: {
        market_id: marketId,
        subaccount_id: subaccountId
      }
    }, fee, memo, _funds);
  };
  register = async ({
    connectionId,
    interchainAccountId
  }: {
    connectionId: string;
    interchainAccountId: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      register: {
        connection_id: connectionId,
        interchain_account_id: interchainAccountId
      }
    }, fee, memo, _funds);
  };
  registerInjectivePositionQuery = async ({
    connectionId,
    marketId,
    subaccountId,
    updatePeriod
  }: {
    connectionId: string;
    marketId: MarketId;
    subaccountId: SubaccountId;
    updatePeriod: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      register_injective_position_query: {
        connection_id: connectionId,
        market_id: marketId,
        subaccount_id: subaccountId,
        update_period: updatePeriod
      }
    }, fee, memo, _funds);
  };
}