#!/usr/bin/env bash
# Creates files with the expanded macros for further analysis

set -e

cd capybara

cargo rustc --features "capybara/python" -- -Z unstable-options --pretty=expanded > ../capybara-python.rs; rustfmt ../capybara-python.rs
cargo rustc --features "capybara/ruby" -- -Z unstable-options --pretty=expanded > ../capybara-ruby.rs; rustfmt ../capybara-ruby.rs
cargo rustc --features "capybara/wasm" --target wasm32-unknown-unknown -- -Z unstable-options --pretty=expanded > ../capybara-wasm.rs; rustfmt ../capybara-wasm.rs

cd ..

cargo rustc --manifest-path helix/Cargo.toml -- -Z unstable-options --pretty=expanded > native-helix.rs; rustfmt native-helix.rs
cargo rustc --manifest-path pyo3/Cargo.toml -- -Z unstable-options --pretty=expanded > native-pyo3.rs; rustfmt native-pyo3.rs
cargo rustc --manifest-path wasm/Cargo.toml --target wasm32-unknown-unknown -- -Z unstable-options --pretty=expanded > native-wasm.rs; rustfmt native-wasm.rs

echo "========== Wasm-bindgen ============"

diff capybara-wasm.rs native-wasm.rs

echo "============= Helix ==============="

diff capybara-ruby.rs native-helix.rs

echo "============== Pyo3 ==============="

diff capybara-python.rs native-pyo3.rs
