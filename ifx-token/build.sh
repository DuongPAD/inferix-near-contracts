#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo +stable build --target wasm32-unknown-unknown --release
# cd ..
cp ../target/wasm32-unknown-unknown/release/ifx_token.wasm ../res/ifx_token.wasm
