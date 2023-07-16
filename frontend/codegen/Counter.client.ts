/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.31.6.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { Coin, StdFee } from "@cosmjs/amino";
import { InstantiateMsg, ExecuteMsg, QueryMsg, GetCountResponse } from "./Counter.types";
export interface CounterReadOnlyInterface {
  contractAddress: string;
  getCount: () => Promise<GetCountResponse>;
}
export class CounterQueryClient implements CounterReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.getCount = this.getCount.bind(this);
  }

  getCount = async (): Promise<GetCountResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_count: {}
    });
  };
}
export interface CounterInterface extends CounterReadOnlyInterface {
  contractAddress: string;
  sender: string;
  increment: (fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  reset: ({
    count
  }: {
    count: number;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
}
export class CounterClient extends CounterQueryClient implements CounterInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.increment = this.increment.bind(this);
    this.reset = this.reset.bind(this);
  }

  increment = async (fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      increment: {}
    }, fee, memo, _funds);
  };
  reset = async ({
    count
  }: {
    count: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      reset: {
        count
      }
    }, fee, memo, _funds);
  };
}