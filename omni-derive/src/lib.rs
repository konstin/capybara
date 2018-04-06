//! A language binding consists of three parts:
//!  - Generating classes from structs
//!  - Generating method bindings from impl blocks
//!  - Generating the FFI-Entrypoint with a macro

#![feature(proc_macro, specialization, const_fn)]
#![recursion_limit = "1024"]

extern crate helix;
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
use std::str::FromStr;
// It was just to much bloat
use syn::*;

struct Pyo3Builder;

struct HelixBuilder;

/// This is stub target that does not emitt any bindings
struct StubBuilder;

/// The heart of omni: This attribute can be added to a struct to generate bindings for that struct,
/// and then also to a plain impl block (i.e. not a trait implementation).
#[proc_macro_attribute]
pub fn omni_bindgen(attr: TokenStream, input: TokenStream) -> TokenStream {
    let builder = get_builder();
    let ast = parse_item(&input.to_string()).unwrap();

    // Ideally, all libraries would use the same stable syn ^1.0 and expose an interface with syn
    // types, so we could parse once and forward the already extracted parts. But for we let all
    // libraries do their own parsing.
    let generated = match ast.node {
        ItemKind::Fn(_, _, _, _, _, _) => panic!("Sorry, omni doesn't support functions yet"),
        ItemKind::ForeignMod(_) => panic!("Sorry, omni doesn't support extern block yet"),
        ItemKind::Enum(_, _) => panic!("Sorry, omni doesn't support enums yet"),
        ItemKind::Struct(_, _) => builder.class(attr.to_string(), input.to_string()),
        ItemKind::Trait(_, _, _, _) => panic!("Sorry, omni doesn't support trait declarations yet"),
        ItemKind::Impl(_, _, _, _, _, _) => builder.methods(attr.to_string(), input.to_string()),
        _ => panic!(
            "You can not generate bindings for this kind of item (Hint: {})",
            ast.ident
        ),
    };

    TokenStream::from_str(&generated).unwrap()
}

/// A workaround for getting feaature-independent typings
fn get_builder() -> &'static BindingBuilder {
    let features = vec![
        cfg!(feature = "use_helix"),
        cfg!(feature = "use_pyo3"),
    ];

    let activated: usize = features.iter().map(|x| *x as usize).sum();
    if activated > 1 {
        panic!("You must activate a single target for omni, not {}", activated);
    }

    if cfg!(feature = "use_helix") {
        return &HelixBuilder;
    } else if cfg!(feature = "use_pyo3") {
        return &Pyo3Builder;
    } else {
        return &StubBuilder;
    }
}

/// A language binding is defined by implementing this on a unit struct and the init macro
///
/// All methods get the stringified versions of what the proc_macro_attribute gets. They have to
/// take a self to make the dynamic dispatch via get_builder() possible
trait BindingBuilder {
    fn class(&self, attr: String, input: String) -> String;
    fn methods(&self, attr: String, input: String) -> String;
}

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
}

/// This is the same boilerplate that pyo3 uses
impl BindingBuilder for Pyo3Builder {
    fn class(&self, attr: String, input: String) -> String {
        let mut ast = parse_derive_input(&input).unwrap();
        let expanded = build_py_class(&mut ast, attr);
        quote!(
            #ast
            #expanded
        ).to_string()
    }

    fn methods(&self, _: String, input: String) -> String {
        let mut ast = parse_item(&input).unwrap();
        Pyo3Builder::add_function_annotations(&mut ast);
        let expanded = build_py_methods(&mut ast);
        quote!(
            #ast
            #expanded
        ).to_string()
    }
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

impl BindingBuilder for StubBuilder {
    fn class(&self, _: String, input: String) -> String {
        return input;
    }

    fn methods(&self, _: String, input: String) -> String {
        return input;
    }
}