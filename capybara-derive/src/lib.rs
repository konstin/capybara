//! A language binding consists of three parts:
//!  - Generating classes from structs
//!  - Generating method bindings from impl blocks
//!  - Generating the FFI-Entrypoint with a macro

#![feature(proc_macro, specialization, const_fn)]
#![recursion_limit = "1024"]

#[cfg(feature = "capybara_ruby")]
extern crate helix;
extern crate proc_macro;
extern crate proc_macro2;
#[cfg(feature = "capybara_python")]
extern crate pyo3;
#[cfg(feature = "capybara_python")]
extern crate pyo3_derive_backend;
#[cfg(not(feature = "quote-0-3"))]
#[macro_use]
extern crate quote;
#[cfg(feature = "quote-0-3")]
#[macro_use]
extern crate quote_0_3;
extern crate syn;
#[cfg(feature = "syn-0-11")]
extern crate syn_0_11;

#[cfg(feature = "capybara_ruby")]
use helix_builder::HelixBuilder;
use proc_macro::TokenStream;
#[cfg(feature = "capybara_python")]
use pyo3_builder::Pyo3Builder;

use stub_builder::StubBuilder;
#[cfg(feature = "capybara_wasm")]
use wasm_builder::WasmBuilder;

#[cfg(feature = "capybara_python")]
mod pyo3_builder;

#[cfg(feature = "capybara_ruby")]
mod helix_builder;

#[cfg(feature = "capybara_wasm")]
mod wasm_builder;

mod stub_builder;

/// The heart of capybara: This attribute can be added to a struct to generate bindings for that struct,
/// and then also to a plain impl block (i.e. not a trait implementation).
#[proc_macro_attribute]
pub fn capybara_bindgen(attr: TokenStream, input: TokenStream) -> TokenStream {
    capybara_bindgen_impl(attr, input)
}

fn capybara_bindgen_impl(attr: TokenStream, input: TokenStream) -> TokenStream {
    let item: syn::Item = syn::parse(input.clone()).unwrap();

    let builder = get_builder();

    let generated = match item {
        syn::Item::ForeignMod(_) => builder.foreign_mod(attr, input),
        syn::Item::Struct(_) => builder.class(attr, input),
        syn::Item::Impl(_) => builder.methods(attr, input),
        _ => panic!("This item of kind isn't supported"),
    };

    generated
}

/// A workaround for getting feaature-independent typings
#[allow(unreachable_code)]
fn get_builder() -> &'static BindingBuilder {
    let features = vec![
        cfg!(feature = "capybara_ruby"),
        cfg!(feature = "capybara_python"),
        cfg!(feature = "capybara_wasm"),
    ];

    let activated: usize = features.iter().map(|x| *x as usize).sum();
    if activated > 1 {
        panic!(
            "You can only generate binding for a single target. Check that you only use a\
             single feature of capybara in your Cargo.toml"
        );
    }

    #[cfg(feature = "capybara_ruby")]
    return &HelixBuilder;
    #[cfg(feature = "capybara_python")]
    return &Pyo3Builder;
    #[cfg(feature = "capybara_wasm")]
    return &WasmBuilder;

    return &StubBuilder;
}

/// A language binding is defined by implementing this on a unit struct and the init macro
///
/// All methods have to take a self to make the dynamic dispatch via get_builder() possible.
trait BindingBuilder {
    /// Gets a struct
    fn class(&self, attr: TokenStream, input: TokenStream) -> TokenStream;
    /// Gets an impl block
    fn methods(&self, attr: TokenStream, input: TokenStream) -> TokenStream;
    /// Gets an extern block
    fn foreign_mod(&self, attr: TokenStream, input: TokenStream) -> TokenStream;
}
