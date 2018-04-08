# capybara

A framework for generating bindings from Rust to arbitrary languages. Currently supports python (via pyo3), ruby
(via helix) and wasm/js (via wasm_bindgen) are supported.

**Note: This is in alpha stage. It can't do much more than static methods yet.**

## Usage

A nightly compiler <= nightly-2018-04-06 is required (see [this bug](https://github.com/rust-lang/rust/issues/49768)).

The boilerplate:

```rust
#![feature(proc_macro, specialization, const_fn)]

#[macro_use]
extern crate capybara;

use capybara::capybara_bindgen;
```

Annotate every class you want to export with `#[class]`, e.g.:

```rust
#[capybara_bindgen]
struct MyClass {}
```

Put the methods to be exported into an impl-block and annotate that block with `#[methods]`

```rust
#[capybara_bindgen]
impl MyClass {
    fn print_and_add(x: i32, y: i32) -> i32 {
        println!("Printing from rust: {}", x + y);
        x + y
    }
}
```

Finally, we need to generate an extrypoint for module/package on the target site. This is done by calling capybara_init!
with the name of module/package and the names of the structs to generate classes form.

```rust
capybara_init! {capybara_test, [MyClass]}
```

Add the following to your Cargo.toml:

```toml
[lib]
name = "<Name of the module you used in capybara_inti!>"
crate-type = ["cdylib"]
```

If only target a single language, you can use the `features` option. "capybara_python" is for python, "capybara_ruby" is for ruby.
Note that these options are mutually exclusive.

```
[dependencies]
capybara = { version = "0.1.0", features = ["capybara_python"] }
```

You can also specify the target language by omitting the features part and instead passing `--features capybara_ruby` to
`cargo build`.

### Constructors

_This feature currently only works with python_

Capybara needs to rewrite your constructors to make them work with the underlying libraries. Therefore a constructor must be called `new`, there must be no `return` statements inside the function and the instance must be built in the last expression of the function. Example:

```
struct MyClass {
    x: usize,
    y: i32,
}

impl MyClass {
    fn new(x: usize) -> MyClass {
        println!("Building an instance");
        MyClass {
            x,
            y: -x,
        }
    }
}
```

### Python (pyo3)

Python is supported through the library pyo3. After running cargo build, copy the generated `lib<module name>.so` and
rename it to `<module name>.so`. You can then `import <module name>`. Hint: Use `<module name>.__dict__` to see what
is in there.

### Ruby on Rails (helix)

Follow [helix' great getting started](https://usehelix.com/getting_started), but replace the lib.rs and cargo
dependencies the ones from this repo.

### Wasm/js (wasm_bindgen)

wasm_bindgen's interface looks essentially the the same way that capybara, so `capybara_bindgen` does essentially the
same as `wasm_bindgen` even though it supports much less featues.
[Read wasm_bindgen's awesome getting started](https://github.com/rustwasm/wasm-bindgen) on how to generate bindings.
Note that `extern` blocks can not use annotations on functions (which are required for e.g. console.log) and that
`println!()` doesn't print for that target, so you effectively must check the return value of a call into rust in the
javascript to see whether everything is set up correctly.

## Features

 * Empty structs
 * Static methods that do not return errors
 * Ruby, Python, wasm and a default stub target

## Missing

 * Constructors
 * Functions (in not methods)
 * A CLI that wraps the wasm-bindgen-cli, setuptools-rust and `rails generate helix:crate text_transform`
 * Windows and Mac OS X (This shouldn't be to much work, mostly porting the tests from bash + python to pure rust)
 * Special methods (equals, comparisons, hashing)
 * Conversions
 * Returning errors
 * Exporting trait implementations
 * Importing via extern blocks
 * Better interface for adding languages

## Testing

There is an integration testing system as simply as the features currently are. Use `test.sh` to run all tests.

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([license-apache](license-apache) or
   http://www.apache.org/licenses/license-2.0)
 * MIT license ([license-mit](LICENSE-mit) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
