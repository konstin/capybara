// It was just to much bloat
use syn::*;

pub struct HelixBuilder;
use super::BindingBuilder;

impl BindingBuilder for HelixBuilder {
    /// Calls codegen_from_struct!
    fn class(&self, _: String, input: String) -> String {
        let item = parse_item(&input).unwrap();
        quote!(codegen_from_struct! {
            #item
        }).to_string()
    }

    /// This parses the methods into a call to codegen_extra_impls!
    fn methods(&self, _: String, input: String) -> String {
        // Really-low-prio-but-would-be-nice: Write this using macros only and then backport to
        // helix
        let mut methods_for_macro = vec![];

        let ast = parse_item(&input).unwrap();
        let rust_name_class;
        if let ItemKind::Impl(_, _, _, None, ref ty, ref methods) = ast.node {
            rust_name_class = quote!(#ty);
            for method in methods {
                let rust_name = &method.ident;
                if let ImplItemKind::Method(ref method_sig, ref body) = &method.node {
                    let input = &method_sig.decl.inputs;
                    let output = match method_sig.decl.output {
                        FunctionRetTy::Default => quote!(()),
                        FunctionRetTy::Ty(ref ty) => quote!(#ty),
                    };

                    let method_type = match method_sig.decl.inputs.get(0) {
                        Some(FnArg::SelfRef(_, _)) | Some(FnArg::SelfValue(_)) => {
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
        } else {
            panic!();
        };

        let generated = quote! {
            #ast
            codegen_extra_impls!({
                type: class,
                rust_name: #rust_name_class,
                ruby_name: { stringify!(#rust_name_class) },
                meta: { pub: true, reopen: false},
                struct: (),
                methods: [#(#methods_for_macro)*]
            });
        }.into_string();

        generated
    }

    fn foreign_mod(&self, _: String, _: String) -> String {
        unimplemented!()
    }
}
