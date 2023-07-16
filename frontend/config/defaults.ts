import { assets } from 'chain-registry';
import { AssetList, Asset } from '@chain-registry/types';

export const chainName = 'neutrontestnet';
export const stakingDenom = 'uneutron';

export const injChainName = 'injectivetestnet'
export const injStakingDenom = 'inj'
// export const feeDenom = 'ujuno';

// export const cw20ContractAddress = 'juno1j0a9ymgngasfn3l5me8qpd53l5zlm9wurfdk7r65s5mg6tkxal3qpgf5se'
export const cw20ContractAddress = 'neutron1pnudnh6t7t3qq5m9sytrvv8mjlq087xxehty46st8n4afjcyn0asfnnxp5'


// export const chainName = 'cosmwasmtestnet';
// export const stakingDenom = 'umlg';
// export const feeDenom = 'uand';

// export const cw20ContractAddress = 'wasm1p7vmrhl3s0fyl0m9hk2hlm7uuxq84hztur63n8ryh85chh30vt6q89shcv'

export const chainassets: AssetList = assets.find(
    (chain) => chain.chain_name === chainName
) as AssetList;

export const injChainAssets: AssetList = assets.find(
    (chain) => chain.chain_name === injChainName
) as AssetList;

export const coin: Asset = chainassets.assets.find(
    (asset) => asset.base === stakingDenom
) as Asset;

export const injCoin: Asset = injChainAssets.assets.find(
    (asset) => asset.base === injStakingDenom
) as Asset;
