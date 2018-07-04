#!/usr/bin/env bash

mkdir -p hello
RUSTFLAGS=-Awarnings cargo +nightly build --target wasm32-unknown-unknown --release
wasm-gc target/wasm32-unknown-unknown/release/hello.wasm -o hello/hello.wasm
wasm2wat --ignore-custom-section-errors hello/hello.wasm > hello/hello.wast
cleos set contract hello.code ./hello -p hello.code
