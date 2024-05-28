#!/bin/bash

set -eE -o functrace

failure() {
  local lineno=$1
  local msg=$2
  echo "Failed at $lineno: $msg"
}
trap 'failure ${LINENO} "$BASH_COMMAND"' ERR

NEUTRON_ADMIN_KEY="admin"
NEUTRON_ADMIN_ADDRESS="neutron1m9l358xunhhwds0568za49mzhvuxx9ux8xafx2"
NEUTRON_USER_KEY="user"
NEUTRON_USER_ADDRESS="neutron10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf"
NEUTRON_CHAIN_ID="test-1"
COSMOS_HUB_USER_KEY="user"
COSMOS_HUB_USER_ADDRESS="cosmos10h9stc5v6ntgeygf5xf945njqq5h32r53uquvw"
COSMOS_HUB_VAL="cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn"
COSMOS_HUB_CHAIN_ID="test-2"
CONNECTION_ID="connection-0"

STORE_HASH=$(neutrond tx wasm store ./artifacts/awesome_restaker-aarch64.wasm --from $NEUTRON_ADMIN_KEY --gas-prices 0.025untrn --gas auto --gas-adjustment 1.75 --chain-id $NEUTRON_CHAIN_ID  --yes --keyring-backend test --output json | jq -r ".txhash")
sleep 3
CODE_ID=$(neutrond q tx "$STORE_HASH" --output json | jq -r '.events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value')
echo "Uploaded contract with code id: $CODE_ID"

INSTANTIATE_HASH=$(neutrond tx wasm instantiate "$CODE_ID" "{\"admin\": \"$NEUTRON_ADMIN_ADDRESS\", \"neutron_register_ica_fee\": \"1000000\"}" --label awesome_restaker --admin $NEUTRON_ADMIN_KEY --from $NEUTRON_ADMIN_KEY --gas-prices 0.025untrn --gas auto --gas-adjustment 1.75 --chain-id $NEUTRON_CHAIN_ID --yes --keyring-backend test --output json | jq -r ".txhash")
sleep 3
CONTRACT_ADDR=$(neutrond q tx "$INSTANTIATE_HASH" --output json | jq -r '.events[] | select(.type=="instantiate") | .attributes[] | select(.key=="_contract_address") | .value')
echo "Instantiated contract with address: $CONTRACT_ADDR"

neutrond tx wasm execute "$CONTRACT_ADDR" "{\"add_supported_chain\": {\"chain_id\": \"$COSMOS_HUB_CHAIN_ID\", \"connection_id\": \"$CONNECTION_ID\"}}" --amount 1000000untrn --from $NEUTRON_ADMIN_KEY --gas-prices 0.025untrn --gas auto --gas-adjustment 1.75 --chain-id $NEUTRON_CHAIN_ID --yes --keyring-backend test --output json
sleep 3
neutrond q wasm contract-state smart neutron1zl6tmh5s4kf0h7k4chkxu2g5x0y5xlw7ylrd7mh7zu28a9jmln3qnstm8w '{"supported_chains": {}}'

gaiad tx staking delegate $COSMOS_HUB_VAL 100000000000uatom --from $COSMOS_HUB_USER_KEY --keyring-backend test --gas-prices 0.025uatom --gas auto --gas-adjustment 1.75 --chain-id $COSMOS_HUB_CHAIN_ID --yes --node tcp://localhost:16657
sleep 3

neutrond tx wasm execute "$CONTRACT_ADDR" "{\"register_user\": {\"registrations\": [{\"chain_id\": \"$COSMOS_HUB_CHAIN_ID\", \"address\": \"$COSMOS_HUB_USER_ADDRESS\"}]}}" --from $NEUTRON_USER_KEY --gas-prices 0.025untrn --gas auto --gas-adjustment 1.75 --chain-id $NEUTRON_CHAIN_ID --yes --keyring-backend test --output json
sleep 3

echo ""
echo "Success!"
echo "Contract deployed at address: $CONTRACT_ADDR"
echo "Supported chain: $COSMOS_HUB_CHAIN_ID"
echo "To check supported chains and ica_address, run:"
echo "neutrond q wasm contract-state smart $CONTRACT_ADDR '{\"supported_chains\": {}}'"
echo "To check validator delegation, run:"
echo "gaiad q staking delegations $COSMOS_HUB_USER_ADDRESS --node tcp://localhost:16657"