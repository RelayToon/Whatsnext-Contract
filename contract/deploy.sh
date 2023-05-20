#!/bin/sh
./scripts/build.sh

LOGFILE="deploy.log"
if [ ! -e '$LOGFILE' ];then
  echo ">> Generating deploy.log"
  touch deploy.log
fi

date +%Y-%m-%d_%T >> deploy.log

if [ $? -ne 0 ]; then
  echo ">> Error building contract" >> deploy.log
  exit 1
fi

echo ">> Deploying contract" >> deploy.log

# If direcotry,"neardev", is persist, all of cotracts is deployed in same account.
DIRECTORY=neardev
if [ -d "$DIRECTORY" ];then
  echo ">> There is neardev \nRemoving Process" >> deploy.log
  rm -rf neardev
fi

# near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/hello_near.wasm | tee -a deploy.log
near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/vote.wasm | tee -a deploy.log
# near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/voteController.wasm | tee -a deploy.log
# near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/community.wasm | tee -a deploy.log
# near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/content.wasm | tee -a deploy.log


# Deploing for Vote Contract is deprecated.
# near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/vote.wasm | tee -a deploy.log

echo "================================================" >> deploy.log