#!/bin/sh

echo ">> Building contract"

cd .. & rustup target add wasm32-unknown-unknown

# cargo build --all --target wasm32-unknown-unknown
cd .. & cargo build --all --target wasm32-unknown-unknown --release
