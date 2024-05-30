# CosmWasm contracts

The contract implements an auto-compounding service for staking rewards over ICA.

It leverages Neutron for ICA and ICQ.

## Flow

![flow](../flow.png)

The contract has a single ICA account on every supported chain that the user will give MsgDelegate Authz permissions to.

The contract leverages the Neutron ICQ module to get all the information it needs to calculate the users pending rewards (it's quite a bit, because the rewards are not actually stored in the state machine (and therefor is currently unavailable for ICQ) and is always calculated on the fly).

There is a single permissionless endpoint that triggers auto-compounding for all the users on all the networks.
It simply checks which users are due for an auto-compounding and sends out Authz Exec with MsgDelegate to the ICA account.

Automated triggering of the endpoint can be done by anyone (it is incentivized),
but the plan is to use either Neutron Cron or an automation network like Warp.
Since the user is paying for the service, paying for this is built into the economics.

## Interesting code

- [reward implementation](https://github.com/InterChadz/awesomewasm-2024/blob/main/cosmwasm/packages/restaker-utils/src/rewards.rs)
    - It is not possible to query pending rewards with ICQ at the moment
    - So we implemented the reward calculation in the contract based on the Cosmos SDK version (https://github.com/cosmos/cosmos-sdk/blob/7009a2e0cdb93b286199bbaa22e8880aeec83c8c/x/distribution/keeper/delegation.go#L58)
- [ICQ key generation](https://github.com/InterChadz/awesomewasm-2024/blob/main/cosmwasm/contracts/restaker/src/icq/keys.rs)
    - Generating ICQ keys is a bit tricky as it needs to be the same binary key as defined in the Cosmos SDK
    - We also need to extract information from keys in certain cases, so we convert and parse the binary keys both ways
- [ICQ reconstruction](https://github.com/InterChadz/awesomewasm-2024/blob/main/cosmwasm/contracts/restaker/src/icq/reconstruct.rs)
    - This is where the ICQ information is parsed and reconstructed into usable data types (from Binary to Structs, essentially - having to parse the keys to find the right data types)
- [ICQ Sudo handling](https://github.com/InterChadz/awesomewasm-2024/blob/main/cosmwasm/contracts/restaker/src/sudo.rs)
    - Some of the ICQ data needs to be updated based on some of the other data
    - So in the `sudo_kv_query_result` method we read the data and update the ICQ query, so we can get all the data we need
- [All the execute stuff](https://github.com/InterChadz/awesomewasm-2024/blob/main/cosmwasm/contracts/restaker/src/execute.rs)
    - `add_supported_chain()` where we deal with adding chains and creating ICA accounts
    - `register_user()` where we register the user and set up the ICQ queries for every chain they want to auto-compound on
    - `autocompound()` where we trigger the auto-compounding, check which users are due for auto-compounding and send out the Authz Exec with MsgDelegate to the ICA account

## Test

Most of the core logic is covered by tests.

To run the tests:
```shell
$ cargo test
```

Some of the tests are commented out due to time-pressure at the end where tests started breaking when hacking quickly :)

## Deploy locally

Prerequisites:

### 1. Make sure Cosmopark is running.

We are using our own fork for comsopark which you can find on our github organization.

[See here for instructions](https://docs.neutron.org/neutron/build-and-run/cosmopark), but swap out the clone with our repos and the branches:

- https://github.com/InterChadz/neutron-query-relayer (branch: `testy-2`)
- https://github.com/InterChadz/neutron (branch: `testy-2`)
- https://github.com/InterChadz/neutron-integration-tests (branch: `testy-2`)

After you've started the chains (the `make start-cosmopark` command), you should wait a little bit for the chains and relayers to start up.

### 2. Neutron and Gaia binaries installed

Just build the binaries from the repos you cloned in the Cosmopark setup so you have `neutrond` and `gaiad` available.

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

(Again, this depends on the Cosmopark setup being done with our forks and running in the background)

```
just optimize
just deploy
```
