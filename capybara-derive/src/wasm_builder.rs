extern crate wasm_bindgen_backend as backend;

use super::BindingBuilder;
use proc_macro::TokenStream;
use quote::{ToTokens, Tokens};
use syn;

#[macro_use]
use quote;

pub struct WasmBuilder;

impl WasmBuilder {
    /// This function is adapted from wasm_bindegen 9723fd, crates/macro/src/lib.rs
    fn actual_impl(&self, _: String, input: String) -> String {
        let item = syn::parse_str::<syn::Item>(&input).expect("expected a valid Rust item");
        // For now no wasm_bindgen attributes are supported
        let opts = backend::ast::BindgenAttrs::default();

        let mut ret = Tokens::new();
        ret.append_all(quote!(use capybara::reexport::*;));
        let mut program = backend::ast::Program::default();
        program.push_item(item, Some(opts), &mut ret);
        program.to_tokens(&mut ret);

        ret.to_string()
    }
}

impl BindingBuilder for WasmBuilder {
    fn class(&self, attr: String, input: String) -> String {
        self.actual_impl(attr, input)
    }

    fn methods(&self, attr: String, input: String) -> String {
        self.actual_impl(attr, input)
    }

    fn foreign_mod(&self, attr: String, input: String) -> String {
        self.actual_impl(attr, input)
    }
}
