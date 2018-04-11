#!/usr/bin/env bash

set -ex

bundle install > /dev/null
DEBUG_RUST=true CARGO_EXTRA_ARGS="--features capybara-ruby" rake test
