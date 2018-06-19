//! Beware, this code is mostly workarounds directly ported from syn 0.11

extern crate pyo3_derive_backend;
extern crate syn;

use super::BindingBuilder;
use proc_macro2::TokenStream;
use syn::buffer::TokenBuffer;
use syn::punctuated::Punctuated;
use syn::token::Comma;

pub struct Pyo3Builder;

fn attribute_from_str(attr_str: &str) -> syn::Attribute {
    let tokens = attr_str.parse().unwrap();
    let buf = syn::buffer::TokenBuffer::new2(tokens);
    syn::Attribute::parse_outer(buf.begin()).unwrap().0
}

/// This is the same boilerplate that pyo3 uses
impl BindingBuilder for Pyo3Builder {
    fn class(&self, attr: TokenStream, input: TokenStream) -> TokenStream {
        let mut class = syn::parse2(input).unwrap();

        let args: Vec<syn::Expr> = {
            let buffer = TokenBuffer::new2(attr);
            let punc = Punctuated::<syn::Expr, Comma>::parse_terminated(buffer.begin());
            punc.expect("could not parse macro arguments")
                .0
                .into_iter()
                .collect()
        };

        let expanded = pyo3_derive_backend::py_class::build_py_class(&mut class, &args);
        let tokens = quote!(
            #class
            #expanded
        );

        tokens.into()
    }

    fn methods(&self, _: TokenStream, input: TokenStream) -> TokenStream {
        let mut impl_block: syn::ItemImpl = syn::parse2(input).expect("Expected an impl block");

        let classname = impl_block.self_ty.clone();
        let constructor = Pyo3Builder::constructor(&mut impl_block.items, &classname);
        Pyo3Builder::add_function_annotations(&mut impl_block.items);

        let rust_new = if let Some((rust_new, pyo3_new, rust_new_pos)) = constructor {
            impl_block.items.insert(rust_new_pos, pyo3_new);

            Some(rust_new)
        } else {
            None
        };

        let expanded =
            pyo3_derive_backend::py_impl::impl_methods(&classname, &mut impl_block.items);

        if let Some(new_method_impl) = rust_new {
            // Add the initial new method back
            quote!(
                #impl_block
                #expanded

                impl #classname {
                    #new_method_impl
                }
            )
        } else {
            quote!(
                #impl_block
                #expanded
            )
        }
    }

    fn foreign_mod(&self, _: TokenStream, _: TokenStream) -> TokenStream {
        unimplemented!()
    }

    fn function(&self, _: TokenStream, input: TokenStream) -> TokenStream {
        let mut item_fn: syn::ItemFn = syn::parse2(input).unwrap();

        let python_name = item_fn.ident.clone();
        let expanded =
            pyo3_derive_backend::module::add_fn_to_module(&mut item_fn, &python_name, Vec::new());

        let tokens = quote!(
            #item_fn
            #expanded
        );

        tokens
    }
}

#[cfg(feature = "python")]
impl Pyo3Builder {
    /// pyo3 expects static methods to be annotated, so let's add that annotation on static methods.
    fn add_function_annotations(impl_items: &mut Vec<syn::ImplItem>) {
        for impl_item in impl_items.iter_mut() {
            if let syn::ImplItem::Method(ref mut method) = impl_item {
                let is_static = match method.sig.decl.inputs.iter().nth(0) {
                    Some(syn::FnArg::SelfRef(_)) => false,
                    Some(syn::FnArg::SelfValue(_)) => false,
                    _ => true,
                };

                // Pyo3 currently can't handle the implicit return type
                if method.sig.decl.output == syn::ReturnType::Default {
                    method.sig.decl.output = parse_quote!(-> ());
                }

                if is_static {
                    method.attrs.push(attribute_from_str("#[staticmethod]"));
                }
            } else {
                panic!("Expected a method");
            }
        }
    }

    /// Builds a pyo3 style constructor from a rust style one, given one does exist
    ///
    /// The rust new will be removed from the method list and returned together with the generated
    /// pyo3 one
    fn constructor(
        impl_items: &mut Vec<syn::ImplItem>,
        classname: &syn::Type,
    ) -> Option<(syn::ImplItem, syn::ImplItem, usize)> {
        let mut rust_new_pos = None;
        let mut rust_new = None;

        for (pos, impl_item) in impl_items.iter().enumerate() {
            if let syn::ImplItem::Method(ref method) = impl_item {
                if method.sig.ident == "new" {
                    rust_new_pos = Some(pos);
                    rust_new = Some(method.clone());
                }
            }
        }

        let rust_new_pos = rust_new_pos?;
        let mut rust_new = rust_new?;

        // pyo3 can't deal with that method, so we remove it
        impl_items.remove(rust_new_pos);

        let contructor_attribute = attribute_from_str("#[capybara_bindgen(constructor)]");

        let attribute_pos = rust_new
            .attrs
            .iter()
            .position(|x| &contructor_attribute == x);

        match attribute_pos {
            None => panic!("A constructor must have a #[capybara_bindgen(constructor)] annotation"),
            Some(pos) => {
                rust_new.attrs.remove(pos);
            }
        };

        match rust_new.block.stmts.last() {
            Some(syn::Stmt::Expr(syn::Expr::Struct(_))) => {}
            _ => panic!("The last expression in a constructor must be instantiating a struct"),
        };

        let args_decl = rust_new.sig.decl.inputs.clone();
        let args_usage: Vec<syn::Pat> = args_decl
            .iter()
            .map(|x| match x {
                syn::FnArg::Captured(captured) => captured.pat.clone(),
                _ => panic!("Argument type not expected in constructor: {:?}", x),
            })
            .collect();

        // I've tried building this with syn primitives but the code became unmanagable,
        // so yes, I'm actually doing this with serializing and deserializing
        // (Now that this code is ported to syn 0.13, there might be a better way to do this)
        let pyo3_new = quote!(
            #[new]
            fn __new__(obj: &PyRawObject, #(#args_decl,)*) -> PyResult<()> {
                obj.init(|_| {
                    #classname::new(#(#args_usage,)*)
                })
            }
        );

        let pyo3_new: syn::ImplItem = syn::parse2(pyo3_new).unwrap();

        Some((syn::ImplItem::Method(rust_new), pyo3_new, rust_new_pos))
    }
}
