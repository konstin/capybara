use proc_macro::TokenStream;
use super::BindingBuilder;
use syn;

pub struct HelixBuilder;

impl BindingBuilder for HelixBuilder {
    /// Calls codegen_from_struct!
    fn class(&self, _: TokenStream, input: TokenStream) -> TokenStream {
        let class: syn::ItemStruct = syn::parse(input).unwrap();

        let tokens = quote!(codegen_from_struct! {
            #class
        });

        tokens.into()
    }

    /// This parses the methods into a call to codegen_extra_impls!
    fn methods(&self, _: TokenStream, input: TokenStream) -> TokenStream {
        // Really-low-prio-but-would-be-nice: Write this using macros only and then backport to
        // helix
        let mut methods_for_macro = vec![];

        let impl_block: syn::ItemImpl = syn::parse(input).unwrap();
        let rust_name_class;
        rust_name_class = impl_block.self_ty.clone();
        for item in &impl_block.items {
            if let syn::ImplItem::Method(method) = item {
                let rust_name = &method.sig.ident;
                let input = &method.sig.decl.inputs;
                let body = &method.block;

                let output = match method.sig.decl.output {
                    syn::ReturnType::Default => quote!(()),
                    syn::ReturnType::Type(_, ref ty) => quote!(#ty),
                };

                let method_type = match method.sig.decl.inputs.first().map(syn::punctuated::Pair::into_value) {
                    Some(syn::FnArg::SelfRef(_)) | Some(syn::FnArg::SelfValue(_)) => {
                        quote!(instance_method)
                    }
                    _ => quote!(class_method),
                };

                methods_for_macro.push(quote!({
                    type: #method_type,
                    rust_name: #rust_name,
                    ruby_name: { stringify!(#rust_name) },
                    self: (),
                    args: [ #(#input),* ] ,
                    ret: { #output },
                    body: #body
                }));
            } else {
                panic!();
            }
        }

        let tokens = quote! {
            #impl_block
            codegen_extra_impls!({
                type: class,
                rust_name: #rust_name_class,
                ruby_name: { stringify!(#rust_name_class) },
                meta: { pub: true, reopen: false},
                struct: (),
                methods: [#(#methods_for_macro)*]
            });
        };

        tokens.into()
    }

    fn foreign_mod(&self, _: TokenStream, _: TokenStream) -> TokenStream {
        unimplemented!()
    }
}
