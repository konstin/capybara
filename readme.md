# Omni

A framework for generating Rust bindings for arbitrary languages. Currently python and ruby are supported.

**Note: This is in alpha stage. It can't do much more than static methods yet.**

## Usage

A nightly compiler is required.

The boilerplate:

```rust
#![feature(proc_macro, specialization, const_fn)]

#[macro_use]
extern crate omni;

use omni::{class, methods};
use omni::prelude::*;
```

Annotate every class you want to export with `#[class]`, e.g.:

```rust
#[class]
struct MyClass {}
```

Put the methods to be exported into an impl-block and annotate that block with `#[methods]`

```rust
#[methods]
impl MyClass {
    fn print_and_double(x: i32) -> i32 {
        println!("Printing from rust: {}", x * 2);
        x * 2
    }
}
```

Finally, we need to generate an extrypoint for module/package on the target site. This is done by calling omni_init!
with the name of module/package and the names of the structs to generate classes form.

```rust
omni_init! {omni_test, [MyClass]}
```

Add the following to your Cargo.toml:

```toml
[lib]
name = "<Name of the module you used in omni_inti!>"
crate-type = ["cdylib"]
```

If only target a single language, you can use the `features` option. "use_pyo3" is for python, "use_helix" is for ruby.
Note that these options are mutually exclusive.

```
[dependencies]
omni = { version = "0.1.0", features = ["use_pyo3"] }
```

You can also specify the target language by omitting the features part and instead passing `--features use_helix` to
`cargo build`.

### Python (pyo3)

Python is supported through the library pyo3. After running cargo build, copy the generated `lib<module name>.so` and
rename it to `<module name>.so`. You can then `import <module name>`. Hint: Use `<module name>.__dict__` to see what
is in there.

### Ruby on Rails (helix)

Follow [helix' getting started guide](https://usehelix.com/getting_started), but replace the lib.rs and cargo
dependencies the ones from this repo.

## Features

 * Empty structs
 * Static methods that do not return errors

## Missing

 * Constructors
 * Special methods (equals, comparisons, hashing)
 * A CLI that wraps the wasm-bindgen-cli, setuptools-rust and `rails generate helix:crate text_transform`
 * Unifying the different attributes to a single one (omni_bindgen?)
 * Integration of wasm_bindgen
 * A default No-op target
 * Functions (in not methods)
 * Exporting trait implementations
 * Windows and Mac OS X (This shouldn't be to much work, mostly making the python test platform agnostic)
 * Conversions
 * Returning errors
 * Importing via extern blocks
 * Better interface for languages

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
