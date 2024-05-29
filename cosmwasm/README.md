# CosmWasm contracts

TODO: NAME

## Deploy locally

Prerequisites:
- Cosmopark running: https://docs.neutron.org/neutron/build-and-run/cosmopark
- Neutron and Gaia binaries installed
- Neutron key named `admin` with mnemonic `banner spread envelope side kite person disagree path silver will brother under couch edit food venture squirrel civil budget number acquire point work mass`
  - `neutrond keys add admin --recover --keyring-backend test`
- Neutron key named `user` with mnemonic `veteran try aware erosion drink dance decade comic dawn museum release episode original list ability owner size tuition surface ceiling depth seminar capable only`
  - `neutrond keys add user --recover --keyring-backend test`
- Gaia key named `user` with mnemonic `veteran try aware erosion drink dance decade comic dawn museum release episode original list ability owner size tuition surface ceiling depth seminar capable only`
  - `gaiad keys add user --recover --keyring-backend test`
  - Run `just optimize` to build the contract artifact
  - Run `just deploy` to deploy the contract

## Work up for grabs:

Later/if time:
- [ ] Implement msg to update a supported chain
- [ ] Implement msg to remove a supported chain
- [ ] Implement msg to update config
- [ ] Find a way to support neutron (local chain) auto compounding
- [ ] Handle ICQs that have expired (how to fill them up (?) and let the user know when they need to do something)