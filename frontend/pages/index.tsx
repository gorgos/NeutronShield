import {
  Box,
  Image,
  useColorMode,
  Menu,
  MenuButton,
  MenuList,
  MenuItem,
  Flex,
  Heading
} from '@chakra-ui/react';
import { BsFillMoonStarsFill, BsFillSunFill } from 'react-icons/bs';
import {
  getDefaultSubaccountId,
} from '@injectivelabs/sdk-ts'

import { useChain } from '@cosmos-kit/react';
import { WalletStatus } from '@cosmos-kit/core';

import { ContractsProvider, useContracts } from '../codegen/contracts-context';
import { indexerDerivativesApi } from '../app/Services';

import {
  chainName,
  injChainName,
  cw20ContractAddress,
  dependencies,
  products,
} from '../config';
import {
  Product,
  Dependency,
  WalletSection,
  handleChangeColorModeValue,
  HackCw20,
  Count,
  Chart,
  MsgGrant,
  AutoTopUp,
} from '../components';
import { useState, useEffect } from 'react';

const ContractComponent = ({ children }: { children: any }) => {
  const { address, getCosmWasmClient, getSigningCosmWasmClient } = useChain(chainName);
  return (
    <ContractsProvider contractsConfig={{
      address,
      getCosmWasmClient,
      getSigningCosmWasmClient
    }}>
      {children}
    </ContractsProvider>
  );
};

const RenderCount = () => {
  const { counter } = useContracts();

  const { address, status } = useChain(chainName);
  const [count, setCount] = useState<number | null>(null);

  if (status === 'Connected' && counter.cosmWasmClient) {
    const client = counter.getQueryClient(cw20ContractAddress);
    client.getCount().then(count => setCount(count.count));
  }

  const handleIncrement = async () => {
    if (status === 'Connected' && counter.cosmWasmClient) {
      const signingClient = counter.getSigningClient(cw20ContractAddress)

      const signed = await signingClient.increment({
        amount: [{
          denom: 'ntrn',
          amount: '300000'
        }],
        gas: '400000'
      });


      const client = counter.getQueryClient(cw20ContractAddress);
      client.getCount().then(count => setCount(count.count));
    }
  }

  return (<Box w="full" maxW="md" mx="auto">
    <Count count={count} isConnectWallet={status !== WalletStatus.Disconnected} onClick={handleIncrement} />
  </Box>)
}

const MarketDropdown = ({ onSelect, marketSelected }) => {
  const [selectedOption, setSelectedOption] = useState("Select Market")
  const [subaccountId, setSubaccountId] = useState('');
  const { status, address } = useChain(injChainName);

  const handleMenuItemClick = (option) => {
    setSelectedOption(option);
    onSelect(option);
  };

  useEffect(() => {
    if (address) {
      setSubaccountId(getDefaultSubaccountId(address || ''))

      indexerDerivativesApi.fetchPositions({
        subaccountId,
      }).then((positions) => {
        console.log({ positions })

        console.log({ positions })
      })
    }
  }, [status]);



  return (
    <Box w="300px" mx="auto" mr="564px" mt="40px">
      <Menu>
        <MenuButton as={Box} p={4} borderWidth={1} borderRadius="md">
          {selectedOption}
        </MenuButton>
        <MenuList>
          <MenuItem onClick={() => handleMenuItemClick("ATOM/USDT Perp")}>ATOM/USDT Perp</MenuItem>
          <MenuItem onClick={() => handleMenuItemClick("INJ/USDT Perp")}>INJ/USDT Perp</MenuItem>
          <MenuItem onClick={() => handleMenuItemClick("BTC/USDT Perp")}>BTC/USDT Perp</MenuItem>
        </MenuList>
      </Menu>
    </Box>
  );
}


const Layout = () => {
  const { colorMode, toggleColorMode } = useColorMode();
  const [isInjectiveConnected, setIsInjectiveConnected] = useState(true)

  const { status, address } = useChain(isInjectiveConnected ? injChainName : chainName);

  const [marketSelected, setMarketSelected] = useState(false);

  const handleSelect = (option) => {
    setMarketSelected(true);
  };

  const [isChecked, setIsChecked] = useState(false);

  const handleToggle = () => {
    setIsChecked(!isChecked);
  }

  return (
    <div>
      <Flex justifyContent="end" alignItems="start" position="relative">
        <Flex justifyContent="end" alignItems="start" mb={4} mt={4} position="absolute">
          <WalletSection isInjectiveConnected={isInjectiveConnected} setIsInjectiveConnected={setIsInjectiveConnected} />
        </Flex>
      </Flex>

      <Box textAlign="center" py={10} paddingRight="40">
        <Flex gap="4" justifyContent="center" alignItems="center" mb={3}>
          <Image
            src="/logo.png"
            boxSize="80px"
          />

          <div>
            <Heading
              as="h1"
              fontSize={{ base: '3xl', sm: '4xl', md: '5xl' }}
              fontWeight="extrabold"

            >
              Neutron Shield
            </Heading>

            <div
              style={{ fontSize: "20px" }}
            >
              InterChain DeFi Management
            </div>
          </div>
        </Flex>
      </Box>

      <Flex justifyContent="center" alignItems="center" mr="200px">
        {status === 'Connected' && isInjectiveConnected && <MsgGrant onChecked={handleToggle} isChecked={isChecked} />}
      </Flex>

      {status === 'Connected' && isChecked && !isInjectiveConnected && <MarketDropdown onSelect={handleSelect} marketSelected={marketSelected} />}

      {marketSelected && isChecked && !isInjectiveConnected ? <Chart /> : <div></div>}
      {marketSelected && isChecked && !isInjectiveConnected ? <AutoTopUp /> : <div></div>}

    </div>
  );
}

export default function Home() {
  return (
    <ContractComponent>
      <Layout />
    </ContractComponent>
  );
}
