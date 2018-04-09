use super::BindingBuilder;

use proc_macro::TokenStream;

/// This is stub target that does not emitt any bindings
pub struct StubBuilder;

impl BindingBuilder for StubBuilder {
    fn class(&self, _: TokenStream, input: TokenStream) -> TokenStream {
        input
    }

    fn methods(&self, _: TokenStream, input: TokenStream) -> TokenStream {
        input
    }

    fn foreign_mod(&self, _: TokenStream, input: TokenStream) -> TokenStream {
        input
    }
}
