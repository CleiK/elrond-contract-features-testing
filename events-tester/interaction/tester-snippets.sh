#!/bin/bash
PROJECT="/home/cleik/Documents/Development/Elrond/simplicity-contracts"
INTERACTION_RESULT="${PROJECT}/interaction/result"
PROXY="https://devnet-gateway.elrond.com"
PROXY_ARGUMENT="--proxy="$PROXY
CHAIN_ARGUMENT="--chain=D"
OUTFILE_POSTFIX="devnet"

DEPLOYER="${PROJECT}/interaction/wallets/deployer.pem"
USER1="${PROJECT}/interaction/wallets/user1.pem"

TESTER_WASM="output/tester.wasm"
DELEGATION_MOCK_WASM="../simplicity-contracts/mocks/delegation-mock/output/delegation-mock.wasm"
DEPLOY_GAS="80000000"
MAX_GAS_LIMIT="600000000"

delegation_mock_deploy() {
  echo "  delegation_mock_deploy.."

  erdpy --verbose contract deploy --bytecode=${DELEGATION_MOCK_WASM} --recall-nonce --pem=${DEPLOYER} \
        --gas-limit=90000000 ${PROXY_ARGUMENT} ${CHAIN_ARGUMENT}\
        --outfile="${INTERACTION_RESULT}/deploy-delegation-mock-${OUTFILE_POSTFIX}.interaction.json" --send --wait-result || return

  DELEGATIONMOCK_DEPLOY_TRANSACTION=$(erdpy data parse --file="${INTERACTION_RESULT}/deploy-delegation-mock-${OUTFILE_POSTFIX}.interaction.json" --expression="data['emittedTransactionHash']")
  DELEGATIONMOCK_CONTRACT_ADDRESS=$(erdpy data parse --file="${INTERACTION_RESULT}/deploy-delegation-mock-${OUTFILE_POSTFIX}.interaction.json" --expression="data['contractAddress']")

  echo ""
  echo "DELEGATIONMOCK smart contract address: ${DELEGATIONMOCK_CONTRACT_ADDRESS} deployed with tx: ${DELEGATIONMOCK_DEPLOY_TRANSACTION}"
}

# --project=${PROJECT}
tester_deploy() {
  echo "  tester_deploy.."

  erdpy --verbose contract deploy --bytecode=${TESTER_WASM} --recall-nonce --pem=${DEPLOYER} \
        --gas-limit=90000000 ${PROXY_ARGUMENT} ${CHAIN_ARGUMENT}\
        --arguments 0x$(erdpy wallet bech32 --decode ${DELEGATIONMOCK_CONTRACT_ADDRESS}) \
        --outfile="${INTERACTION_RESULT}/deploy-tester-${OUTFILE_POSTFIX}.interaction.json" --send --wait-result || return

  TESTER_DEPLOY_TRANSACTION=$(erdpy data parse --file="${INTERACTION_RESULT}/deploy-tester-${OUTFILE_POSTFIX}.interaction.json" --expression="data['emittedTransactionHash']")
  TESTER_CONTRACT_ADDRESS=$(erdpy data parse --file="${INTERACTION_RESULT}/deploy-tester-${OUTFILE_POSTFIX}.interaction.json" --expression="data['contractAddress']")

  echo ""
  echo "TESTER smart contract address: ${TESTER_CONTRACT_ADDRESS} deployed with tx: ${TESTER_DEPLOY_TRANSACTION}"
}


tester_event() {
  erdpy --verbose contract call ${TESTER_CONTRACT_ADDRESS} --recall-nonce --pem=${USER1} --gas-limit=3700000 \
    --function="test_event" --value=0 --send ${PROXY_ARGUMENT} ${CHAIN_ARGUMENT} --wait-result
}

tester_double_event() {
  erdpy --verbose contract call ${TESTER_CONTRACT_ADDRESS} --recall-nonce --pem=${USER1} --gas-limit=5000000 \
    --function="test_double_event" --value=0 --send ${PROXY_ARGUMENT} ${CHAIN_ARGUMENT} --wait-result
}

tester_event_in_callback() {
  erdpy --verbose contract call ${TESTER_CONTRACT_ADDRESS} --recall-nonce --pem=${USER1} --gas-limit=8000000 \
    --function="test_event_in_callback" --value=0 --send ${PROXY_ARGUMENT} ${CHAIN_ARGUMENT} --wait-result
}

tester_double_event_in_callback() {
  erdpy --verbose contract call ${TESTER_CONTRACT_ADDRESS} --recall-nonce --pem=${USER1} --gas-limit=5100000 \
    --function="test_double_event_in_callback" --value=0 --send ${PROXY_ARGUMENT} ${CHAIN_ARGUMENT} --wait-result
}


