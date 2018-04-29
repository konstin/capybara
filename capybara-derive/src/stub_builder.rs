use super::BindingBuilder;
use proc_macro::TokenStream;
use quote::{ToTokens, Tokens};
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
        let mut impl_block: syn::ItemImpl = syn::parse(input).unwrap();

        struct Walk;

        impl<'ast> syn::visit_mut::VisitMut for Walk {
            fn visit_impl_item_method_mut(&mut self, method: &mut syn::ImplItemMethod) {
                method.attrs = method
                    .attrs
                    .clone()
                    .into_iter()
                    .filter(|attr| attr.path != syn::parse_str("capybara_bindgen").unwrap())
                    .collect();
            }
        }

        syn::visit_mut::VisitMut::visit_item_impl_mut(&mut Walk, &mut impl_block);

        let mut tokens = Tokens::new();
        impl_block.to_tokens(&mut tokens);
        tokens.into()
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
