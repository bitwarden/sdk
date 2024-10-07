#!/bin/sh

# Build the wasm library
cd ../../crates/bitwarden-wasm
npm i -g binaryen
cargo install wasm-bindgen-cli
sh build.sh

# Build the TS client
cd ../../languages/js/sdk-client
npm ci
npm run build

# Actually run the tests
cd ../e2e-test
npm ci
npm run test
