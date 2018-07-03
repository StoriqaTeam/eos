#!/usr/bin/env bash

mkdir -p hello
cargo +nightly build --target wasm32-unknown-unknown --release
wasm-gc target/wasm32-unknown-unknown/release/hello.wasm -o hello/hello.wasm
wasm2wat --ignore-custom-section-errors hello/hello.wasm > hello/hello.wast
cleos --wallet-url http://localhost:8899 set contract hello.code ./hello -p hello.code
