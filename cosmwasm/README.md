# CosmWasm contracts

TODO: NAME

## Deploy locally

Prerequisites:

### 1. Make sure Cosmopark is running.

[See here for instructions](https://docs.neutron.org/neutron/build-and-run/cosmopark).

### 2. Neutron and Gaia binaries installed

TODO: Add more details

### 3. Create a Neutron key named `admin`

```
neutrond keys add admin --recover --keyring-backend
```

Use mnemonic `banner spread envelope side kite person disagree path silver will brother under couch edit food venture squirrel civil budget number acquire point work mass`

### 4. Create a Neutron key named `user`

```
neutrond keys add admin --recover --keyring-backend
```

Use mnemonic `veteran try aware erosion drink dance decade comic dawn museum release episode original list ability owner size tuition surface ceiling depth seminar capable only`

### 5. Create a Gaia key named `user`

```
gaiad keys add user --recover --keyring-backend test
```

Use mnemonic `veteran try aware erosion drink dance decade comic dawn museum release episode original list ability owner size tuition surface ceiling depth seminar capable only`

### 6. Build and deploy the contract

```
just optimize
just deploy
```

## Work up for grabs:

Later/if time:

- [ ] Implement msg to update a supported chain
- [ ] Implement msg to remove a supported chain
- [ ] Implement msg to update config
- [ ] Find a way to support neutron (local chain) auto compounding
- [ ] Handle ICQs that have expired (how to fill them up (?) and let the user know when they need to do something)
