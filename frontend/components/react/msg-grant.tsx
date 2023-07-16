import { useChain } from '@cosmos-kit/react';
import { DeliverTxResponse, SigningStargateClient, StargateClient, StdFe } from '@cosmjs/stargate';
import {
  chainName,
  injChainName,
  cw20ContractAddress,
  dependencies,
  products,
} from '../../config';
import {
  Flex,
  Text,
  Switch,
  useToast
} from '@chakra-ui/react';
import {
  MsgSend,
  MsgGrant as MsgGrantSdk,
} from '@injectivelabs/sdk-ts'
import { msgBroadcastClient } from '../../app/Services';
import { walletStrategy } from '../../app/wallet-strategy';
import { getNetworkEndpoints, Network } from '@injectivelabs/networks';
import { useState } from 'react';

export const MsgGrant = ({ isChecked, onChecked }) => {
  const toast = useToast();


  const { address, getSigningStargateClient } = useChain(injChainName);
  const sendMsgGrant = async () => {
    walletStrategy.setWallet('keplr')
    console.log({ walletStrategy })
    const endpoints = getNetworkEndpoints(Network.Testnet)
    const granteeAddress = address
    const granterAddress = 'inj1c7duyzd5mkraa7xyj98fm4pcfq59unzf63x6rx'

    const message = MsgSend.fromJSON({
      srcInjectiveAddress: granteeAddress,
      dstInjectiveAddress: granterAddress,
      amount: {
        denom: 'inj',
        amount: '1000000'
      }
    })

    const result = await msgBroadcastClient.broadcastOld({
      injectiveAddress: address,
      msgs: message
    })

    if (result) {
      toast({
        title: "Success!",
        // description: "We've created your account for you.",
        status: "success",
        duration: 3000,
        isClosable: true,
        position: "bottom-right",
      })
    }
  }

  const handleToggled = () => {
    if (!isChecked) {
      onChecked()

      sendMsgGrant()
    }
  }

  return (
    <Flex alignItems="center" mt="80px">
      <Text fontSize="16px" fontWeight="semibold" marginRight="10px">
        Grant Permission to Neutron ICA on Injective
      </Text>
      <Switch
        size="lg"
        colorScheme="teal"
        isChecked={isChecked}
        onChange={handleToggled}
      />
    </Flex>
  );
};
