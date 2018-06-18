use super::BindingBuilder;
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn;

/// This is stub target that does not emitt any bindings
pub struct StubBuilder;

impl BindingBuilder for StubBuilder {
    /// no-op
    fn class(&self, _: TokenStream, input: TokenStream) -> TokenStream {
        input
    }

    /// Removes all the capybara_bindgen attributes
    fn methods(&self, _: TokenStream, input: TokenStream) -> TokenStream {
        let mut impl_block: syn::ItemImpl = syn::parse2(input).unwrap();

        struct Walk;

        impl<'ast> syn::visit_mut::VisitMut for Walk {
            fn visit_impl_item_method_mut(&mut self, method: &mut syn::ImplItemMethod) {
                // For some unknown reason parse_quote fails here
                let path_segment = syn::PathSegment {
                    ident: syn::Ident::new("capybara_bindgen", Span::call_site()),
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

        syn::visit_mut::VisitMut::visit_item_impl_mut(&mut Walk, &mut impl_block);

        impl_block.into_token_stream()
    }

    /// no-op
    fn foreign_mod(&self, _: TokenStream, input: TokenStream) -> TokenStream {
        input
    }

    /// no-op
    fn function(&self, _: TokenStream, input: TokenStream) -> TokenStream {
        input
    }
}
