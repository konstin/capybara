#!/bin/bash

# Testing with the browser is cumbersome and slow, so this uses node instead

set -ex

cargo build --target wasm32-unknown-unknown --features capybara-wasm

wasm-bindgen ../target/wasm32-unknown-unknown/debug/capybara_test.wasm --out-dir . --nodejs --debug

node index.js