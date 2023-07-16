/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.31.6.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { ContractBase, IContractConstructor, IEmptyClient } from "./contractContextBase";
import { NeutronShieldClient, NeutronShieldQueryClient } from "./NeutronShield.client";
export class NeutronShield extends ContractBase<NeutronShieldClient, NeutronShieldQueryClient, IEmptyClient> {
  constructor({
    address,
    cosmWasmClient,
    signingCosmWasmClient
  }: IContractConstructor) {
    super(address, cosmWasmClient, signingCosmWasmClient, NeutronShieldClient, NeutronShieldQueryClient, undefined);
  }

}