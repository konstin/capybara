#!/usr/bin/env bash

set -ex

bundle install > /dev/null
DEBUG_RUST=true rake test

