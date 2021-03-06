extern crate wasm_bindgen_backend as backend;
extern crate wasm_bindgen_macro_support as macro_support;

use self::macro_support::expand;
use super::BindingBuilder;
use proc_macro2::{Span, TokenStream};
use syn;

const ATTRIBUTE_SELF: &str = "capybara";
const ATTRIBUTE_WASM: &str = "wasm_bindgen";

/// changes `#[capybara(...)]` into `#[wasm_bindgen(...)]`
struct AttributeTransformer;

impl<'ast> syn::visit_mut::VisitMut for AttributeTransformer {
    fn visit_attribute_mut(&mut self, attr: &mut syn::Attribute) {
        if let Some(ref mut first_segment) = attr.path.segments.iter_mut().nth(0) {
            if first_segment.ident == ATTRIBUTE_SELF {
                first_segment.ident = syn::Ident::new(ATTRIBUTE_WASM, Span::call_site());
            }
        }
    }
}

pub struct WasmBuilder;

impl WasmBuilder {
    fn wasm_impl(&self, _: TokenStream, mut item: syn::Item) -> TokenStream {
        syn::visit_mut::VisitMut::visit_item_mut(&mut AttributeTransformer, &mut item);

        match expand(quote!(), quote!(#item)) {
            Ok(tokens) => tokens.into(),
            Err(diagnostic) => (quote! { #diagnostic }).into(),
        }
    }
}

impl BindingBuilder for WasmBuilder {
    fn class(&self, attr: TokenStream, class: syn::ItemStruct) -> TokenStream {
        self.wasm_impl(attr, syn::Item::Struct(class))
    }

    fn methods(&self, attr: TokenStream, input: syn::ItemImpl) -> TokenStream {
        self.wasm_impl(attr, syn::Item::Impl(input))
    }

    fn foreign_mod(&self, attr: TokenStream, input: syn::ItemForeignMod) -> TokenStream {
        self.wasm_impl(attr, syn::Item::ForeignMod(input))
    }

    fn function(&self, attr: TokenStream, input: syn::ItemFn) -> TokenStream {
        self.wasm_impl(attr, syn::Item::Fn(input))
    }
}
