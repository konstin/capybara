#!/usr/bin/env bash
# Runs the test.sh in every integration test folder

set -ex

cd tests

for I in ./*; do
    cd $I; ./test; cd ..
done
