#!/bin/bash

set -o errexit -o nounset -o pipefail

STORE_HASH=$(neutrond tx wasm store ./artifacts/awesome_restaker-aarch64.wasm --from admin --gas-prices 0.025untrn --gas auto --gas-adjustment 1.75 --chain-id test-1  --yes --keyring-backend test --output json | jq -r ".txhash")
sleep 3
CODE_ID=$(neutrond q tx "$STORE_HASH" --output json | jq -r '.events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value')
echo "Uploaded contract with code id: $CODE_ID"

INSTANTIATE_HASH=$(neutrond tx wasm instantiate "$CODE_ID" '{}' --label awesome_restaker --admin admin --from admin --gas-prices 0.025untrn --gas auto --gas-adjustment 1.75 --chain-id test-1 --yes --keyring-backend test --output json | jq -r ".txhash")
sleep 3
CONTRACT_ADDR=$(neutrond q tx "$INSTANTIATE_HASH" --output json | jq -r '.events[] | select(.type=="instantiate") | .attributes[] | select(.key=="_contract_address") | .value')
echo "Instantiated contract with address: $CONTRACT_ADDR"