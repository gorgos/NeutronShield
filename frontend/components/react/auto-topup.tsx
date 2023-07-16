import React from "react";
import { Flex, Switch, Box, Button, Heading, Stack, Text, useColorMode } from "@chakra-ui/react";
import { handleChangeColorModeValue } from "./handleChangeColor";
import { ContractsProvider, useContracts } from '../../codegen/contracts-context';
import { useState } from 'react';
export const AutoTopUp = () => {
  const contractAddress = 'neutron1w3m8qu9j68nrjfcutpdntrw3m2wsp4qwmqvgss3vdkuyhcyuj0ushlxphf'
  const { neutronShield } = useContracts();

  const [isChecked, setIsChecked] = useState(false);

  const handleTopUp = async () => {

    const signingClient = neutronShield.getSigningClient(contractAddress)

    await signingClient.registerInjectivePositionQuery({
      connection_id: '1',
      market_id: '0x123',
      subaccount_id: '0x456',
      update_period: 3
    }, {
      amount: [{
        denom: 'ntrn',
        amount: '300000'
      }],
      gas: '400000'
    },)

    // await signingClient.register({ connectionId: '1', interchainAccountId: '1' }, {
    //   amount: [{
    //     denom: 'ntrn',
    //     amount: '300000'
    //   }],
    //   gas: '400000'
    // },)
  }

  const handleToggled = () => {
    setIsChecked(!isChecked);

    if (!isChecked) {
      handleTopUp()
    }
  }

  return (
    <Flex alignItems="center" mt="80px" justifyContent="center" marginRight="200px">
      <Text fontSize="16px" fontWeight="semibold" marginRight="10px">
        Auto Top Up Position
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
