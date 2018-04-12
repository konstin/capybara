#!/usr/bin/env bash
# Runs the test.sh in every integration test folder

set -ex

# Those builds will be reused by the tests

cargo build
cargo build --features python
cargo build --features ruby
cargo build --features wasm --target wasm32-unknown-unknown

cd tests

for I in ./test-*;
    do $I;
done
