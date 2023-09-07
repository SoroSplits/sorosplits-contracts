echo "1. Addding futurenet to soroban config \n"
soroban config network add --global futurenet \
  --rpc-url https://rpc-futurenet.stellar.org:443 \
  --network-passphrase "Test SDF Future Network ; October 2022"

echo "2. Creating new identity called "sorosplit-test" \n"
soroban config identity generate --global sorosplit-test

export TEST_IDENTITY=$(soroban config identity address sorosplit-test)
echo "3. New identity "sorosplit-test" created: $(echo $TEST_IDENTITY) \n"
printf "%s" "$TEST_IDENTITY" > scripts/test_identity

curl "https://friendbot-futurenet.stellar.org/?addr=$(echo $TEST_IDENTITY)"
echo "\n\n4. Funding the new identity with friendbot. \n"

echo "5. Deploying the splitter contract to the network"
export SPLITTER_CONTRACT_ID=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/sorosplit_splitter.wasm \
  --source sorosplit-test \
  --network futurenet)

echo "Contract deployed. Contract ID: $SPLITTER_CONTRACT_ID\n"
printf "%s" "$SPLITTER_CONTRACT_ID" > scripts/splitter_contract_id

echo "6. Deploying the token contract to the network."
export TOKEN_CONTRACT_ID=$(soroban contract deploy \
  --wasm token_contract.wasm \
  --source sorosplit-test \
  --network futurenet)

echo "Contract deployed. Contract ID: $TOKEN_CONTRACT_ID\n"
printf "%s" "$TOKEN_CONTRACT_ID" > scripts/token_contract_id

exit 0