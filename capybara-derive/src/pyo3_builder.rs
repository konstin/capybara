// This mod uses syn 0.11

use proc_macro::TokenStream;
use pyo3_derive_backend::py_class::build_py_class;
use pyo3_derive_backend::py_impl::build_py_methods;
use std::str::FromStr;

// It was just to much bloat
use super::BindingBuilder;
use syn::*;

pub struct Pyo3Builder;

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

    fn foreign_mod(&self, _: String, _: String) -> String {
        unimplemented!()
    }
}

#[cfg(feature = "capybara_python")]
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
