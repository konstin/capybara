#!/bin/bash

# This should fail, so we discard the error and invert the return code
cargo build --features "capybara-ruby capybara-python" 2> /dev/null && exit 1 || exit 0