extern crate wasm_bindgen_backend as backend;

use super::BindingBuilder;
use proc_macro::TokenStream;
use quote::{ToTokens, Tokens};
use syn;

pub struct WasmBuilder;

impl WasmBuilder {
    /// This function is adapted from wasm_bindegen 9723fd, crates/macro/src/lib.rs
    fn actual_impl(&self, _: TokenStream, input: TokenStream) -> TokenStream {
        let item = syn::parse::<syn::Item>(input).expect("expected a valid Rust item");
        // For now no wasm_bindgen attributes are supported
        let opts = backend::ast::BindgenAttrs::default();

        let mut ret = Tokens::new();
        ret.append_all(quote!(use capybara::reexport::*;));
        let mut program = backend::ast::Program::default();
        program.push_item(item, Some(opts), &mut ret);
        program.to_tokens(&mut ret);

        ret.into()
    }
}

impl BindingBuilder for WasmBuilder {
    fn class(&self, attr: TokenStream, input: TokenStream) -> TokenStream {
        self.actual_impl(attr, input)
    }

    fn methods(&self, attr: TokenStream, input: TokenStream) -> TokenStream {
        self.actual_impl(attr, input)
    }

    fn foreign_mod(&self, attr: TokenStream, input: TokenStream) -> TokenStream {
        self.actual_impl(attr, input)
    }
}
