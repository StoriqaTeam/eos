#!/usr/bin/env bash

mkdir -p review_test
RUSTFLAGS=-Awarnings cargo +nightly build --target wasm32-unknown-unknown --release
wasm-gc ../../target/wasm32-unknown-unknown/release/review_test.wasm -o review_test/review_test.gc.wasm
wasm-opt -Os -o review_test/review_test.wasm review_test/review_test.gc.wasm
wasm2wat --ignore-custom-section-errors review_test/review_test.wasm > review_test/review_test.wast
cleos set contract hello.code ./review_test -p hello.code
