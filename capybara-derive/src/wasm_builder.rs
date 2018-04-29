extern crate wasm_bindgen_backend as backend;

use super::BindingBuilder;
use proc_macro::TokenStream;
use quote::{ToTokens, Tokens};
use syn;

pub struct WasmBuilder;

impl WasmBuilder {
    /// changes `#[capybara_bindgen(...)]` into `#[wasm_bindgen(...)]`
    fn transform_attributes(&self, item: &mut syn::Item) {
        struct Walk;

        impl<'ast> syn::visit_mut::VisitMut for Walk {
            fn visit_attribute_mut(&mut self, attr: &mut syn::Attribute) {
                let first_ident = attr.path.segments.iter().nth(0).map(|x| x.ident);
                if first_ident == Some("capybara_bindgen".into()) {
                    let mut x = syn::punctuated::Punctuated::new();
                    x.push_value(syn::parse_str("wasm_bindgen").unwrap());
                    attr.path.segments = x;
                }
            }
        }

        syn::visit_mut::VisitMut::visit_item_mut(&mut Walk, item);
    }

    /// This function is adapted from wasm_bindegen 9723fd, crates/macro/src/lib.rs
    fn actual_impl(&self, _: TokenStream, input: TokenStream) -> TokenStream {
        let mut item = syn::parse::<syn::Item>(input).expect("expected a valid Rust item");
        self.transform_attributes(&mut item);
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

    fn function(&self, attr: TokenStream, input: TokenStream) -> TokenStream {
        self.actual_impl(attr, input)
    }
}
