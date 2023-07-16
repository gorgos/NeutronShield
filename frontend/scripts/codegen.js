const { join, resolve }  = require('path');
const codegen = require('@cosmwasm/ts-codegen').default;

const contractsDir = resolve(join(__dirname, '/../schemas'));
const contracts = [
  {
    name: 'HackCw20',
    dir: join(contractsDir, 'cw20-base')
  },
  {
    name: 'Counter',
    dir: join(contractsDir, 'counter')
  },
  {name: 'NeutronShield', dir: join(contractsDir, 'neutron-shield')},
];

codegen({
  contracts,
  outPath: join(__dirname, '../codegen'),
  options: {
    bundle: {
      enabled: true,
      bundleFile: 'index.ts',
      scope: 'contracts'
    },
    types: {
      enabled: true
    },
    client: {
      enabled: true
    },
    useContractsHooks: {
      enabled: true
    }
  }
}).then(() => {
  console.log('✨ all done!');
}).catch(e=>{
  console.error(e);
  process.exit(1)
});
