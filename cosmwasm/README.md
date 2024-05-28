# CosmWasm contracts

TODO: NAME

## Deploy locally

Prerequisites:
- Cosmopark running: https://docs.neutron.org/neutron/build-and-run/cosmopark
- Neutron binary
- Neutron key named `admin` with mnemonic `banner spread envelope side kite person disagree path silver will brother under couch edit food venture squirrel civil budget number acquire point work mass`
  - `neutrond keys add admin --recover --keyring-backend test`
- Neutron key named `user` with mnemonic `veteran try aware erosion drink dance decade comic dawn museum release episode original list ability owner size tuition surface ceiling depth seminar capable only`
  - `neutrond keys add user --recover --keyring-backend test`
- Gaia key named `user` with mnemonic `veteran try aware erosion drink dance decade comic dawn museum release episode original list ability owner size tuition surface ceiling depth seminar capable only`
  - `gaiad keys add user --recover --keyring-backend test` 

## Work up for grabs:

- [ ] Implement query to get supported chains
- [ ] Generate typescript types for the contract

Later/if time:
- [ ] Implement msg to update a supported chain
- [ ] Implement msg to remove a supported chain
- [ ] Implement msg to update config
- [ ] Find a way to support neutron (local chain) auto compounding