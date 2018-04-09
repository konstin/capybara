#!/usr/bin/env bash
# Runs the test.sh in every integration test folder

set -ex

# Those build will be reused by the tests

cargo build
cargo build --features capybara_python
cargo build --features capybara_ruby
cargo build --features capybara_wasm --target wasm32-unknown-unknown

cd tests

for I in ./*; do
    cd $I; ./test; cd ..
done
