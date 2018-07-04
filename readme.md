# Capybara

A framework for generating bindings from Rust to arbitrary languages. Currently supports python (via [pyo3](https://github.com/PyO3/pyo3)), ruby
(via [helix](https://github.com/tildeio/helix)) and wasm/js (via [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)) are supported.

**Note: This is in alpha stage. You can't do much more than structs, methods and functions with the basic types yet.**

## Usage

The boilerplate:

```rust
#![feature(proc_macro, specialization, wasm_import_module, wasm_custom_section, concat_idents)]

#[macro_use]
extern crate capybara;

use capybara::prelude::*;
```

Annotate every struct, impl-block and function you want to export with `#[capybara]`:

```rust
#[capybara]
fn double(x: usize) -> usize {
    x * 2
}

#[capybara]
pub struct ExportedClass {}

#[capybara]
impl ExportedClass {
    fn print_and_add(x: i32, y: i32) -> i32 {
        println!("Printing from rust: {}", x + y);
        x + y
    }
}
```

We also need to generate an extrypoint for module/package/gem on the target site. This is done by calling `capybara_init!`
with the name of module/package and the names of the structs to generate classes form.

```rust
capybara_init! {capybara_test, [ExportedClass], [double]}
```

Add the following to your Cargo.toml:

```toml
[lib]
name = "<Name of the module you used in capybara_init!>"
crate-type = ["cdylib"]
```

If you only target a single language, you can use the `features` option. Available are "python", "ruby" and "wasm".
Note that these options are mutually exclusive.

```toml
[dependencies]
capybara = { version = "0.1.0", features = ["python"] }
```

To select the language for each build, omit the features part and pass the target language with `--features ` to
`cargo build`, e.g. with `cargo build --features capbyrar/ruby`.

### Python (pyo3)

Python is supported through a library called pyo3. After running cargo build, copy the generated `lib<module name>.so` from the target folder and
rename it to `<module name>.so`. You can then `import <module name>`. Hint: Use `<module name>.__dict__` to see what
is in there.

### Ruby on Rails (helix)

Follow [helix' great getting started](https://usehelix.com/getting_started), but replace the lib.rs and cargo
dependencies wth the ones from this repo. You also need to patch `project.rb` in the `helix_runtime` gem with [this project.rb](https://github.com/konstin/helix/blob/538a1c9fa9382c85aed50794d91fd6096c2ab6a0/ruby/lib/helix_runtime/project.rb) until [tildeio/helix#148](https://github.com/tildeio/helix/pull/148) is finally merged.

### Wasm/js (wasm-bindgen)

wasm-bindgen's interface looks essentially the the same way that capybara, so `capybara` does essentially the
same as `wasm_bindgen` even though it supports much less featues.
[Read wasm-bindgen's awesome getting started](https://github.com/rustwasm/wasm-bindgen) on how to generate bindings.
Note that `extern` blocks can not use annotations on functions (which are required for e.g. console.log) and that
`println!()` doesn't print for that target, so you effectively must check the return value of a call into rust in the
javascript to see whether everything is set up correctly.

## Design Goals

The main goal is making capybara as _intuitive_ as possible, meaning that you can develop crossing ffi boundaries the same way that you normally develop.

 * Existing code should only need minimal annotations work with capybara.
 * Don't make users think (about the ffi machinery)
 * As many language features as possible should be supported and common features should be bridged. (E.g. Add in rust should be mappend to `__add__` in python and def + in ruby)
 * If the code isn't compiled to an ffi target, all capybara functions should become a no-op.
 * Compatibility with existing tools and workflows, while filling the missing parts with custom tools

## Features

 * Structs declarations with and without fields
 * methods, both static and taking &self, that do not return errors
 * Ruby, Python, wasm and a default stub target
 * Constructors (See restrictions below)

## Missing - Contributions welcome :)

 * Functions in ruby
 * Lift restrictions on constructors: Allow arbitrary returns by traversing the ast with syn's Fold trait
 * Add checks: `crate-type = ["cdylib"]`, items must be `pub`, etc.
 * A CLI that wraps the wasm-bindgen-cli, setuptools-rust and `rails generate helix:crate text_transform`
 * Rewrite test.sh in rust
   * Add the ability to test various toolchains
   * Test all types for usage in struct field, method arguments and return types
 * Export docstrings (This already works with pyo3)
 * Windows and Mac OS X (The proc macro itself should work, the tests should pass on mac os x)
 * Special methods (equals, comparisons, hashing)
 * Conversions
 * Returning errors
 * Exporting trait implementations
 * Importing via extern blocks
 * Review the BindingBuilder trait for better interface options
 * Accessors for public fields outside of wasm_bindgen

## Advanced Usage

### Constructors

Capybara needs to rewrite your constructors to make them work with the underlying libraries. Therefore a constructor must be called `new`, there must be no `return` statements inside the function and the instance must be built in the last expression of the function. Example:

```
#[capybara]
pub struct ExportedClass {
    x: usize,
    y: usize,
}

#[capybara]
impl ExportedClass {
    #[capybara]
    fn new(x: usize) -> ExportedClass {
        println!("Building an instance");
        ExportedClass {
            x,
            y: x+3,
        }
    }
}
```

## Debugging your application

You can view expanded code with the following command, or at least get a macro trace for helix. You might need to have rustfmt installed.

```bash
cargo rustc -- -Z unstable-options --pretty=expanded -Z trace-macros > expanded.rs; rustfmt expanded.rs
```

## Testing

There is an integration testing system checking compilation and different kinds of method signatures for all three target. Use `test.sh` to run all tests.

There is also `compare-codegen/expand.sh`, which allows easily comparing the code generated by capybara with the one generated by invocing the underlying libraries (helix, pyo3 and wasm-bindgen) directly. It will both create files to compare in the tool of your choice and a simple diff output.

### Contributing

Please run `cargo fmt` and - if you are on linux or mac - `./test.sh`.

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([license-apache](license-apache) or
   http://www.apache.org/licenses/license-2.0)
 * MIT license ([license-mit](license-mit) or
   http://opensource.org/licenses/MIT)

at your option.


