//! A language binding consists of three parts:
//!  - Generating classes from structs
//!  - Generating method bindings from impl blocks
//!  - Generating the FFI-Entrypoint with a macro

#![feature(proc_macro, specialization, const_fn)]
#![recursion_limit = "1024"]

extern crate proc_macro;
extern crate proc_macro2;

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro2::TokenStream;

#[cfg(feature = "python")]
mod pyo3_builder;

#[cfg(feature = "ruby")]
mod helix_builder;

#[cfg(feature = "wasm")]
mod wasm_builder;

mod stub_builder;

/// The heart of capybara: This attribute can be added to a struct, its impl block and freestanding
/// functions to create bindings for those items
///
/// You can define a language binding by implementing [BindingBuilder] on a unit struct and the
/// init macro in the capybara crate
#[proc_macro_attribute]
pub fn capybara_bindgen(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    capybara_bindgen_impl(attr, input)
}

/// Dispatches the tokenstream into the right builder method
fn capybara_bindgen_impl(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item: syn::Item = syn::parse(input.clone()).unwrap();

    let builder = get_builder();

    let generated = match item {
        syn::Item::ForeignMod(foreign_mod) => builder.foreign_mod(attr.into(), foreign_mod),
        syn::Item::Struct(class) => builder.class(attr.into(), class),
        syn::Item::Impl(methods) => builder.methods(attr.into(), methods),
        syn::Item::Fn(function) => builder.function(attr.into(), function),
        _ => panic!("This kind of item isn't supported"),
    };

    if cfg!(feature = "debug-macros") {
        print_token_stream(generated.clone(), 0);
    }

    generated.into()
}

/// A workaround for getting feaature-independent typings
#[allow(unreachable_code)]
fn get_builder() -> &'static BindingBuilder {
    let features = vec![
        cfg!(feature = "ruby"),
        cfg!(feature = "python"),
        cfg!(feature = "wasm"),
    ];

    let activated: usize = features.iter().map(|x| *x as usize).sum();
    if activated > 1 {
        panic!(
            "You can only generate binding for a single target. Check that you only use a\
             single feature of capybara in your Cargo.toml"
        );
    }

    #[cfg(feature = "ruby")]
    return &helix_builder::HelixBuilder;
    #[cfg(feature = "python")]
    return &pyo3_builder::Pyo3Builder;
    #[cfg(feature = "wasm")]
    return &wasm_builder::WasmBuilder;

    return &stub_builder::StubBuilder;
}

/// All methods have to take a self to make the dynamic dispatch via get_builder() possible.
trait BindingBuilder {
    /// Implements bindings for a struct
    fn class(&self, attr: TokenStream, class: syn::ItemStruct) -> TokenStream;
    /// Implements bindings for an impl block
    fn methods(&self, attr: TokenStream, methods: syn::ItemImpl) -> TokenStream;
    /// Implements bindings for an extern block
    fn foreign_mod(&self, attr: TokenStream, foreign_mod: syn::ItemForeignMod) -> TokenStream;
    /// Implements bindings for a freestanding function
    fn function(&self, attr: TokenStream, function: syn::ItemFn) -> TokenStream;
}

/// Prints macros with the debug-macros feature
fn print_token_stream(tokens: TokenStream, level: usize) {
    for token in tokens.into_iter() {
        match token {
            proc_macro2::TokenTree::Group(ref group) => {
                let (open, close) = match group.delimiter() {
                    proc_macro2::Delimiter::Parenthesis => ("(", ")"),
                    proc_macro2::Delimiter::Brace => ("{", "}"),
                    proc_macro2::Delimiter::Bracket => ("(", ")"),
                    proc_macro2::Delimiter::None => ("Ø", "Ø"),
                };
                println!("{:>2}: {:<30} {:?}", level, open, token.span());
                print_token_stream(group.stream(), level + 1);
                println!("{:>2}: {:<30} {:?}", level, close, token.span());
            }
            _ => {
                println!("{:>2}: {:<30} {:?}", level, token, token.span());
            }
        };
    }
}
