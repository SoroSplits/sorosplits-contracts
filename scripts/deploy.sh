#!/bin/bash

SOROBAN_RPC_URL="https://soroban-testnet.stellar.org"
SOROBAN_NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
NETWORK="testnet"

echo "1. Addding testnet to soroban config \n"
soroban config network add --global testnet \
  --rpc-url "$SOROBAN_RPC_URL" \
  --network-passphrase "$SOROBAN_NETWORK_PASSPHRASE"

echo "2. Creating new wallet called "sorosplits-wallet" \n"
soroban config identity generate --global sorosplits-wallet \
  --rpc-url "$SOROBAN_RPC_URL" \
  --network-passphrase "$SOROBAN_NETWORK_PASSPHRASE" \
  --network "$NETWORK"

export SOROSPLITS_WALLET=$(soroban config identity address sorosplits-wallet)
echo "3. New wallet "sorosplit-wallet" created: $(echo $SOROSPLITS_WALLET) \n"
printf "%s" "$SOROSPLITS_WALLET" > scripts/artifacts/sorosplits_wallet

curl "https://friendbot.stellar.org/?addr=$(echo $SOROSPLITS_WALLET)"
echo "\n\n4. Funding the new wallet with friendbot. \n"

echo "5. Building contracts \n"
soroban contract build

echo "6. Uploding the splitter contract to the network\n"
export SPLITTER_CONTRACT_WASM_HASH=$(soroban contract install \
  --wasm target/wasm32-unknown-unknown/release/sorosplits_splitter.wasm \
  --source sorosplits-wallet \
  --network testnet)
printf "%s" "$SPLITTER_CONTRACT_WASM_HASH" > scripts/artifacts/splitter_contract_wasm_hash

echo "7. Deploying the deployer contract to the network\n"
export DEPLOYER_CONTRACT_ID=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/sorosplits_deployer.wasm \
  --source sorosplits-wallet \
  --network testnet)
printf "%s" "$DEPLOYER_CONTRACT_ID" > scripts/artifacts/deployer_contract_id

echo "8. Deploying the token contract to the network\n"
export TOKEN_CONTRACT_ID=$(soroban contract deploy \
  --wasm token_contract.wasm \
  --source sorosplits-wallet \
  --network testnet)
printf "%s" "$TOKEN_CONTRACT_ID" > scripts/artifacts/token_contract_id

echo "9. Contract deployment complete. \n\n"
echo "Contract details: \n"

echo "Splitter Contract Wasm Hash: $SPLITTER_CONTRACT_WASM_HASH\n"
echo "Deployer Contract ID: $DEPLOYER_CONTRACT_ID\n"
echo "Token Contract ID: $TOKEN_CONTRACT_ID\n"

exit 0