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
COSMOS_HUB_CHAIN_ID="testy-2"
CONNECTION_ID="connection-0"
AUTOCOMPOUND_COST=100000 # 0.1 $NTRN

# Determine the architecture and set the appropriate file path
if [ "$(uname -m)" = "arm64" ]; then
  wasm_file="./artifacts/awesome_restaker-aarch64.wasm"
else
  wasm_file="./artifacts/awesome_restaker.wasm"
fi

# Store the hash using the appropriate wasm file
STORE_HASH=$(neutrond tx wasm store $wasm_file --from $NEUTRON_ADMIN_KEY --gas-prices 0.025untrn --gas auto --gas-adjustment 1.75 --chain-id $NEUTRON_CHAIN_ID --yes --keyring-backend test --output json | jq -r ".txhash")
sleep 5
CODE_ID=$(neutrond q tx "$STORE_HASH" --output json | jq -r '.events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value')
echo "Uploaded contract with code id: $CODE_ID"

INSTANTIATE_HASH=$(neutrond tx wasm instantiate "$CODE_ID" "{\"admin\": \"$NEUTRON_ADMIN_ADDRESS\", \"neutron_register_ica_fee\": \"1000000\", \"autocompound_threshold\": 100}" --label awesome_restaker --admin $NEUTRON_ADMIN_KEY --from $NEUTRON_ADMIN_KEY --gas-prices 0.025untrn --gas auto --gas-adjustment 1.75 --chain-id $NEUTRON_CHAIN_ID --yes --keyring-backend test --output json | jq -r ".txhash")
sleep 5
CONTRACT_ADDR=$(neutrond q tx "$INSTANTIATE_HASH" --output json | jq -r '.events[] | select(.type=="instantiate") | .attributes[] | select(.key=="_contract_address") | .value')
echo "Instantiated contract with address: $CONTRACT_ADDR"

ADD_CHAIN_HASH=$(neutrond tx wasm execute "$CONTRACT_ADDR" "{\"add_supported_chain\": {\"chain_id\": \"$COSMOS_HUB_CHAIN_ID\", \"connection_id\": \"$CONNECTION_ID\", \"denom\": \"uatom\", \"autocompound_cost\": \"$AUTOCOMPOUND_COST\"}}" --amount 1000000untrn --from $NEUTRON_ADMIN_KEY --gas-prices 0.025untrn --gas auto --gas-adjustment 1.75 --chain-id $NEUTRON_CHAIN_ID --yes --keyring-backend test --output json  | jq -r ".txhash")
sleep 5
ADD_CHAIN_RESULT=$(neutrond q tx "$ADD_CHAIN_HASH" --output json | jq -r ".code")
if [ "$ADD_CHAIN_RESULT" != "0" ]; then
  echo "Error adding supported chain: $ADD_CHAIN_RESULT (tx: $ADD_CHAIN_HASH)"
  exit 1
fi

DELEGATE_HASH=$(gaiad tx staking delegate $COSMOS_HUB_VAL 100000000000uatom --from $COSMOS_HUB_USER_KEY --keyring-backend test --gas-prices 0.025uatom --gas auto --gas-adjustment 1.75 --chain-id $COSMOS_HUB_CHAIN_ID --yes --node tcp://localhost:16657 --output json | jq -r ".txhash")
sleep 5
DELEGATE_RESULT=$(gaiad q tx "$DELEGATE_HASH" --node tcp://localhost:16657 --output json | jq -r ".code")
if [ "$DELEGATE_RESULT" != "0" ]; then
  echo "Error delegating to validator: $DELEGATE_RESULT (tx: $DELEGATE_HASH)"
  exit 1
fi

# REGISTER_USER_HASH=$(neutrond tx wasm execute "$CONTRACT_ADDR" "{\"register_user\": {\"registrations\": [{\"chain_id\": \"$COSMOS_HUB_CHAIN_ID\", \"address\": \"$COSMOS_HUB_USER_ADDRESS\", \"validators\": [\"$COSMOS_HUB_VAL\"]}]}}" --amount 1000000untrn --from $NEUTRON_USER_KEY --gas-prices 0.025untrn --gas auto --gas-adjustment 1.75 --chain-id $NEUTRON_CHAIN_ID --yes --keyring-backend test --output json   | jq -r ".txhash")
# sleep 5
# REGISTER_USER_RESULT=$(neutrond q tx "$REGISTER_USER_HASH" --output json | jq -r ".code")
# if [ "$REGISTER_USER_RESULT" != "0" ]; then
#   echo "Error registering user: $REGISTER_USER_RESULT (tx: $REGISTER_USER_HASH)"
#   exit 1
# fi

echo ""
echo "Success!"
echo "Contract deployed at address: $CONTRACT_ADDR"
echo "Supported chain: $COSMOS_HUB_CHAIN_ID"
echo "To check supported chains and ica_address, run:"
echo "neutrond q wasm contract-state smart $CONTRACT_ADDR '{\"supported_chains\": {}}'"
echo "To check validator delegation, run:"
echo "gaiad q staking delegations $COSMOS_HUB_USER_ADDRESS --node tcp://localhost:16657"
echo "To check user registration, run:"
echo "neutrond q wasm contract-state smart $CONTRACT_ADDR '{\"user_registrations\": {\"address\": \"$NEUTRON_USER_ADDRESS\"}}'"
echo "To check user calculated rewards, run:"
echo "neutrond q wasm contract-state smart $CONTRACT_ADDR '{\"calculate_reward\": {\"address\": \"$NEUTRON_USER_ADDRESS\", \"chain_id\": \"$COSMOS_HUB_CHAIN_ID\", \"remote_address\": \"$COSMOS_HUB_USER_ADDRESS\"}}'"
echo "To check user actual rewards on gaia, run:"
echo "gaiad q distribution rewards $COSMOS_HUB_USER_ADDRESS --node tcp://:16657"
echo "To check user query data, run:"
echo "neutrond q wasm contract-state smart $CONTRACT_ADDR '{\"user_query\": {\"address\": \"$NEUTRON_USER_ADDRESS\", \"chain_id\": \"$COSMOS_HUB_CHAIN_ID\", \"remote_address\": \"$COSMOS_HUB_USER_ADDRESS\"}}'"
echo "To check due users to autocompound, run:"
echo "neutrond q wasm contract-state smart $CONTRACT_ADDR '{\"due_user_chain_registrations\": {\"delegators_amount\": 5}}'"
echo ""
echo "Sanity check for rewards, run:"
echo "echo \"ours:\" && neutrond q wasm contract-state smart $CONTRACT_ADDR '{\"calculate_reward\": {\"address\": \"$NEUTRON_USER_ADDRESS\", \"chain_id\": \"$COSMOS_HUB_CHAIN_ID\", \"remote_address\": \"$COSMOS_HUB_USER_ADDRESS\"}}' --output json | jq -r \".data.rewards[0].reward[0].amount\" && echo \"actual:\" && gaiad q distribution rewards $COSMOS_HUB_USER_ADDRESS --node tcp://:16657 --output json | jq -r \".rewards[0].reward[0].amount\""
echo ""
echo "To trigger auto compound, run:"
echo "neutrond tx wasm execute $CONTRACT_ADDR '{\"autocompound\": {\"delegators_amount\": 5}}' --amount 200000untrn --from $NEUTRON_USER_KEY --gas-prices 0.025untrn --gas auto --gas-adjustment 1.75 --chain-id $NEUTRON_CHAIN_ID --yes --keyring-backend test"
echo ""
echo "First thing first, you need to authz the ICA. Find the ICA address by running:"
echo "neutrond q wasm contract-state smart $CONTRACT_ADDR '{\"supported_chains\": {}}'"
echo "Then, run:"
echo "gaiad tx authz grant INSERT_ICA_ACCOUNT_HERE delegate --allowed-validators $COSMOS_HUB_VAL --from $COSMOS_HUB_USER_KEY --keyring-backend test --gas auto --gas-adjustment 2 --gas-prices 0.025uatom --chain-id testy-2 --node tcp://:16657"