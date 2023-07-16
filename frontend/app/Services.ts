import { MsgBroadcaster } from '@injectivelabs/wallet-ts'
import {  walletStrategy } from './wallet-strategy'
import {
  Network,
  getNetworkEndpoints
} from '@injectivelabs/networks'
import {IndexerGrpcDerivativesApi} from '@injectivelabs/sdk-ts'

const network = Network.TestnetK8s

const endpoints = getNetworkEndpoints(network)

export const indexerDerivativesApi = new IndexerGrpcDerivativesApi(
  endpoints.indexer
)

export const msgBroadcastClient = new MsgBroadcaster({
  walletStrategy,
  network,
  networkEndpoints: endpoints,
  feePayerPubKey: '',
  simulateTx: true
})
