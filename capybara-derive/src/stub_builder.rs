//! Stub builder which is used when no language is targeted. Mostly no-op

use super::BindingBuilder;
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn;

/// This is stub builder that does not emitt any bindings but removes the capybara-attributes
pub struct StubBuilder;

impl BindingBuilder for StubBuilder {
    /// no-op
    fn class(&self, _: TokenStream, class: syn::ItemStruct) -> TokenStream {
        class.into_token_stream()
    }

    /// Removes all the capybara_bindgen attributes
    fn methods(&self, _: TokenStream, mut impl_block: syn::ItemImpl) -> TokenStream {
        struct AttrRemover;

        impl<'ast> syn::visit_mut::VisitMut for AttrRemover {
            fn visit_impl_item_method_mut(&mut self, method: &mut syn::ImplItemMethod) {
                // For some unknown reason parse_quote fails here
                let path_segment = syn::PathSegment {
                    ident: syn::Ident::new("capybara", Span::call_site()),
                    arguments: syn::PathArguments::None,
                };
                let path: syn::Path = path_segment.into();
                method.attrs = method
                    .attrs
                    .clone()
                    .into_iter()
                    .filter(|attr| attr.path != path)
                    .collect();
            }
        }

        syn::visit_mut::VisitMut::visit_item_impl_mut(&mut AttrRemover, &mut impl_block);

        impl_block.into_token_stream()
    }

    /// no-op
    fn foreign_mod(&self, _: TokenStream, input: syn::ItemForeignMod) -> TokenStream {
        input.into_token_stream()
    }

    /// no-op
    fn function(&self, _: TokenStream, input: syn::ItemFn) -> TokenStream {
        input.into_token_stream()
    }
}
