//! A language binding consists of three parts:
//!  - Generating classes from structs
//!  - Generating method bindings from impl blocks
//!  - Generating the FFI-Entrypoint with a macro

#![feature(proc_macro, specialization, const_fn)]
#![recursion_limit = "1024"]
#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
extern crate helix;
#[macro_use]
extern crate log;
extern crate proc_macro;
extern crate pyo3;
extern crate pyo3_derive_backend;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use pyo3_derive_backend::py_class::build_py_class;
use pyo3_derive_backend::py_impl::build_py_methods;
use quote::{ToTokens, Tokens};
use std::str::FromStr;
// It was just to much bloat
use syn::*;

#[derive(PartialEq)]
struct Pyo3Builder;

#[derive(PartialEq)]
struct HelixBuilder;

/// A list of all available language bindings
#[derive(PartialEq)]
enum OmniTarget {
    Pyo3(Pyo3Builder),
    Helix(HelixBuilder),
}

#[cfg(all(feature = "use_helix", feature = "use_pyo3"))]
compile_error!("You can't use helix and pyo3 at the same time.");

#[cfg(feature = "use_helix")]
const MY_TARGET: OmniTarget = OmniTarget::Helix(HelixBuilder);

#[cfg(feature = "use_pyo3")]
const MY_TARGET: OmniTarget = OmniTarget::Pyo3(Pyo3Builder);

/// A language binding is defined by implementing this on a unit struct and the init macro
///
/// All methods get the stringified versions of what the proc_macro_attribute gets
trait BindingBuilder {
    fn class(attr: String, input: String) -> String;
    fn methods(attr: String, input: String) -> String;
}

impl BindingBuilder for HelixBuilder {
    /// Calls codegen_from_struct!
    fn class(_: String, input: String) -> String {
        let item = parse_item(&input).unwrap();
        quote!(codegen_from_struct! {
            #item
        }).to_string()
    }

    /// This parses the methods into a call to codegen_extra_impls!
    fn methods(_: String, input: String) -> String {
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
}

/// This is the same boilerplate that pyo3 uses
impl BindingBuilder for Pyo3Builder {
    fn class(attr: String, input: String) -> String {
        let mut ast = parse_derive_input(&input).unwrap();
        let expanded = build_py_class(&mut ast, attr);
        quote!(
            #ast
            #expanded
        ).to_string()
    }

    fn methods(_: String, input: String) -> String {
        let mut ast = parse_item(&input).unwrap();
        Pyo3Builder::add_function_annotations(&mut ast);
        let expanded = build_py_methods(&mut ast);
        quote!(
            #ast
            #expanded
        ).to_string()
    }
}

/// This can by added to a struct to generate bindings for that struct
#[proc_macro_attribute]
pub fn class(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr = attr.to_string();
    let input = input.to_string();

    let generated = match MY_TARGET {
        OmniTarget::Helix(HelixBuilder) => HelixBuilder::class(attr, input),
        OmniTarget::Pyo3(Pyo3Builder) => Pyo3Builder::class(attr, input),
    };

    TokenStream::from_str(&generated).unwrap()
}

/// This can be added to a plain impl-block. The struct must have the `#[class]` attribute or
/// have the same functionality manually implemented
#[proc_macro_attribute]
pub fn methods(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr = attr.to_string();
    let input = input.to_string();

    let generated = match MY_TARGET {
        OmniTarget::Helix(HelixBuilder) => HelixBuilder::methods(attr, input),
        OmniTarget::Pyo3(Pyo3Builder) => Pyo3Builder::methods(attr, input),
    };

    TokenStream::from_str(&generated).unwrap()
}

impl Pyo3Builder {
    /// pyo3 expects static methods to be annotated, so let's add that annotation on static methods.
    fn add_function_annotations(ast: &mut Item) {
        if let ItemKind::Impl(_, _, _, _, _, ref mut impl_items) = ast.node {
            for method in impl_items.iter_mut() {
                let is_static;

                if let ImplItemKind::Method(ref mut method_sig, _) = method.node {
                    is_static = match method_sig.decl.inputs.get(0) {
                        Some(FnArg::SelfRef(_, _)) => false,
                        Some(FnArg::SelfValue(_)) => false,
                        _ => true,
                    };
                } else {
                    panic!("Expected a method");
                }

                if is_static {
                    method.attrs.push(Attribute {
                        style: AttrStyle::Outer,
                        value: MetaItem::Word(Ident::new("staticmethod")),
                        is_sugared_doc: false,
                    });
                }
            }
        } else {
            panic!("Expected an impl block")
        }
    }
}
