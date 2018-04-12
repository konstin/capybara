#!/usr/bin/env bash
# Creates files with the expanded macros for further analysis

cd capybara

cargo rustc --features "capybara/python" -- -Z unstable-options --pretty=expanded > ../capybara-python.rs; rustfmt ../capybara-python.rs
cargo rustc --features "capybara/ruby" -- -Z unstable-options --pretty=expanded > ../capybara-ruby.rs; rustfmt ../capybara-ruby.rs
cargo rustc --features "capybara/wasm" --target wasm32-unknown-unknown -- -Z unstable-options --pretty=expanded > ../capybara-wasm.rs; rustfmt ../capybara-wasm.rs

cd ..

for I in "helix" "pyo3" "wasm"; do
    cd $I
    cargo rustc -- -Z unstable-options --pretty=expanded > ../native-$I.rs; rustfmt ../native-$I.rs
    cd ..
done

echo "========== Wasm-bindgen ============"

diff capybara-wasm.rs native-wasm.rs

echo "============= Helix ==============="

diff capybara-ruby.rs native-helix.rs

echo "============== Pyo3 ==============="

diff capybara-python.rs native-pyo3.rs
