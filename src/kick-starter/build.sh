#!/bin/bash
cargo build --release --features generate-api-description --target=wasm32-unknown-unknown
wasm-build target kick-starter --target-runtime=substrate --final=kick-starter --save-raw=./target/kick-starter-deployed.wasm --target wasm32-unknown-unknown
