use super::BindingBuilder;
use proc_macro::TokenStream;
use quote::Tokens;
use syn;

pub struct HelixBuilder;

/// The name of the field added by helix, ...
const INSTANTIATION_PARAM: &'static str = "helix";
/// ... which is also passed to the constructor
const INITIALIZE_PARAM: &'static str = "helix: ::Metadata";
/// Despite being a variable in the macros, helix has this name hardcoded for the name of the
/// constructor
const INITIALIZE_HELIX: &'static str = "initialize";

impl HelixBuilder {
    /// Parses the function into the form codegen_ruby_init wants
    fn parse_into_macro_part(&self, method: &syn::ImplItemMethod, is_new: bool) -> Tokens {
        let output = match method.sig.decl.output {
            syn::ReturnType::Default => quote!(()),
            syn::ReturnType::Type(_, ref ty) => quote!(#ty),
        };

        let first = method
            .sig
            .decl
            .inputs
            .first()
            .map(syn::punctuated::Pair::into_value);

        let method_type = match first {
            Some(syn::FnArg::SelfRef(_)) => quote!(instance_method),
            Some(syn::FnArg::SelfValue(_)) => quote!(instance_method),
            _ => if is_new {
                quote!(initializer)
            } else {
                quote!(class_method)
            },
        };

        let inputs = &method.sig.decl.inputs;
        // Helix expects the argument list to be without self
        let args = if method_type == quote!(instance_method) {
            let inputs = inputs.iter().skip(1);
            quote!([#(#inputs),*])
        } else {
            quote!([#(#inputs),*])
        };

        let self_tt = match first {
            Some(syn::FnArg::SelfRef(ref arg_self_ref)) => {
                if arg_self_ref.mutability.is_some() {
                    quote!({
                        ownership: {&mut},
                        name: self
                    })
                } else {
                    quote!({
                        ownership: {&},
                        name: self
                    })
                }
            }
            Some(syn::FnArg::SelfValue(_)) => quote!({
                ownership: {},
                name: self
            }),
            _ => if is_new {
                quote!({
                    ownership: { },
                    name: helix
                })
            } else {
                quote!(())
            },
        };

        let rust_name = &method.sig.ident;
        let body = &method.block;

        quote!({
            type: #method_type,
            rust_name: #rust_name,
            ruby_name: { stringify!(#rust_name) },
            self: #self_tt,
            args: #args ,
            ret: { #output },
            body: #body
        })
    }

    /// Mainly rewriting the new function into the initialize function helix wants
    fn method(&self, mut method: syn::ImplItemMethod) -> (syn::ImplItemMethod, Tokens) {
        let is_new = method.sig.ident == syn::Ident::from("new");

        if is_new {
            method.sig.ident = syn::Ident::from(INITIALIZE_HELIX);

            super::remove_constructor_attribute(&mut method);
        }

        let tokens = self.parse_into_macro_part(&method, is_new);

        if is_new {
            let mut last = method
                .block
                .stmts
                .pop()
                .expect("The new function must have at least one statement");
            match last {
                syn::Stmt::Expr(syn::Expr::Struct(ref mut expr_struct)) => expr_struct
                    .fields
                    .insert(0, syn::parse_str(INSTANTIATION_PARAM).unwrap()),
                _ => panic!(
                    "The last statement of a function must be the instantiation of the struct"
                ),
            };
            method.block.stmts.push(last);

            method
                .sig
                .decl
                .inputs
                .insert(0, syn::parse_str(INITIALIZE_PARAM).unwrap());
        }

        (method, tokens)
    }
}

impl BindingBuilder for HelixBuilder {
    /// Calls codegen_from_struct!
    fn class(&self, _: TokenStream, input: TokenStream) -> TokenStream {
        let class: syn::ItemStruct = syn::parse(input).unwrap();
        let rust_name = &class.ident;
        let struct_body = class.struct_token;

        let extra_codegen_body = quote!({
            type: class,
            rust_name: #rust_name,
            ruby_name: { stringify!(#rust_name) },
            meta: { pub: true, reopen: false},
            struct: #struct_body,
            methods: []
        });

        let tokens = quote!(
            codegen_from_struct! {
                #class
            }

            codegen_coercions!(#extra_codegen_body);
            codegen_allocator!(#extra_codegen_body);
        );

        tokens.into()
    }

    /// Handles some parsing boilerplate and the invocation of codegen_ruby_init!. The actual work
    /// is done by [HelixBuilder::method] and [HelixBuild::parse_into_macro_part]
    fn methods(&self, _: TokenStream, input: TokenStream) -> TokenStream {
        let mut impl_block: syn::ItemImpl = syn::parse(input).unwrap();
        let rust_name = impl_block.self_ty.clone();

        let mut methods_tokens = vec![];
        let mut methods_asts = vec![];
        for item in impl_block.items {
            if let syn::ImplItem::Method(method) = item.clone() {
                let (method_ast, tokens) = self.method(method.clone());
                methods_asts.push(method_ast);
                methods_tokens.push(tokens);
            } else {
                panic!("Only methods are supported in impl block");
            }
        }

        impl_block.items = methods_asts.drain(..).map(|method| method.into()).collect();

        let class = quote!({
            type: class,
            rust_name: #rust_name,
            ruby_name: { stringify!(#rust_name) },
            meta: { pub: true, reopen: false},
            // This will generate the code for structs with extra fields, which works with both
            // empty structs (which got a helix metdata field added) and structs with data.
            //
            // codegen_class_binding and the functions of ClassDefinition it invokes don't
            // have any documentation, so I honestly have no idea if there are any side effects of
            // this. (If you by any chance know what's going on, please ping me)
            struct: {},
            methods: [#(#methods_tokens)*]
        });

        let tokens = quote! {
            #impl_block

            codegen_ruby_init!(#class);
        };

        tokens.into()
    }

    fn foreign_mod(&self, _: TokenStream, _: TokenStream) -> TokenStream {
        unimplemented!()
    }

    fn function(&self, attr: TokenStream, input: TokenStream) -> TokenStream {
        unimplemented!()
    }
}
