# Frontend (change me)

To install: 
```bash
$ npm install
```

To run: 
```bash
$ npm start
```

You need a .env file like this:

This is set up for the forked Cosmopark set detailed in the cosmwasm README.md.

```
# Frontend
BASE_URL=http://localhost:8080
VUE_APP_INTERVAL_TIMEOUT=10000

# Chain
VUE_APP_RPC=http://localhost:26657
VUE_ATOM_RPC=http://localhost:16657

VUE_APP_CHAIN_INFO='{"chainId":"test-1","chainName":"Neutron","rpc":"http://localhost:26657","rest":"http://localhost:1317","bip44":{"coinType":118},"bech32Config":{"bech32PrefixAccAddr":"neutron","bech32PrefixAccPub":"neutronpub","bech32PrefixValAddr":"neutronvaloper","bech32PrefixValPub":"neutronvaloperpub","bech32PrefixConsAddr":"neutronvalcons","bech32PrefixConsPub":"neutronvalconspub"},"currencies":[{"coinDenom":"NTRN","coinMinimalDenom":"untrn","coinDecimals":6,"coinGeckoId":"neutron"}],"feeCurrencies":[{"coinDenom":"NTRN","coinMinimalDenom":"untrn","coinDecimals":6,"coinGeckoId":"neutron","gasPriceStep":{"low":0.0025,"average":0.025,"high":0.04}}],"stakeCurrency":{"coinDenom":"NTRN","coinMinimalDenom":"untrn","coinDecimals":6,"coinGeckoId":"neutron"}}'

VUE_APP_CHAIN_ID=test-1
VUE_APP_FEE_DENOM=untrn
VUE_APP_BASE_FEE=0.0025
VUE_APP_EXPLORER_BASE_URL=https://neutron.celat.one

VUE_APP_CONTRACT=neutron1nxshmmwrvxa2cp80nwvf03t8u5kvl2ttr8m8f43vamudsqrdvs8qqvfwpj

VUE_APP_CHAINS_APIS='[{"chainId":"testy-2","rest":"http://localhost:1316","rpc":"http://localhost:16657","prefix":"cosmos","symbol":"ATOM"}]'
```