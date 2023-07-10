#!/usr/bin/env sh

NODE_ADDRESS=127.0.0.1:9988

# Store the CW20 contract
printf '%s\n' '--- STORE CONTRACT ---'
STORE_CODE_OUTPUT=$(
  ccw substrate --node ws://$NODE_ADDRESS \
    --from alice --output json \
    tx store ./artifacts/bank_query.wasm
)

sleep 6

printf '\n%s\n' '--- STORE CODE RESPONSE ---'
echo "$STORE_CODE_OUTPUT"

CODE_ID=$(echo "$STORE_CODE_OUTPUT" | jq '.extrinsic.details.code_id')

# Instantiate the CW20 contract
printf '\n%s\n' '--- INSTANTIATE CONTRACT ---'
INSTANTIATE_OUTPUT=$(
  ccw substrate --node ws://$NODE_ADDRESS \
    --from alice --output json tx instantiate2 "$CODE_ID" \
    '{}' \
    0x9999 --label 0x1111 --gas 10000000000
)

sleep 6

printf '\n%s\n' '--- INSTANTIATE CONTRACT RESPONSE ---'
echo "$INSTANTIATE_OUTPUT"

CONTRACT_ADDRESS=$(echo "$INSTANTIATE_OUTPUT" | jq '.cosmwasm_events[0].contract' -r)

# Query Alice's balance
printf '\n%s\n' '--- QUERY ALICE PICA BALANCE ---'
ccw substrate --node http://$NODE_ADDRESS \
  --output json query wasm --contract "$CONTRACT_ADDRESS" \
  --gas 10000000000 \
  --query '{"balance": {"address": "5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL", "denom": "PICA"} }'
