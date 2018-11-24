// Beware, this code still includes some syn-0.11 artifacts

extern crate pyo3_derive_backend;
extern crate syn;

use super::BindingBuilder;
use proc_macro2::TokenStream;
use syn::parse::Parser;
use syn::punctuated::Punctuated;

pub struct Pyo3Builder;

/// This is the same boilerplate that pyo3 uses
impl BindingBuilder for Pyo3Builder {
    fn class(&self, attr: TokenStream, mut class: syn::ItemStruct) -> TokenStream {
        let parser = Punctuated::<syn::Expr, Token![,]>::parse_terminated;
        let error_message = "The macro attributes should be a list of comma separated expressions";
        let args = parser
            .parse(attr.into())
            .expect(error_message)
            .into_iter()
            .collect();

        let expanded = pyo3_derive_backend::py_class::build_py_class(&mut class, &args);
        quote!(
            #class
            #expanded
        )
    }

    fn methods(&self, _: TokenStream, mut impl_block: syn::ItemImpl) -> TokenStream {
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

    fn foreign_mod(&self, _: TokenStream, _: syn::ItemForeignMod) -> TokenStream {
        unimplemented!()
    }

    fn function(&self, _: TokenStream, item_fn: syn::ItemFn) -> TokenStream {
        let python_name = item_fn.ident.clone();
        let expanded =
            pyo3_derive_backend::module::add_fn_to_module(&item_fn, &python_name, Vec::new());

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
                    method.attrs.push(parse_quote!(#[staticmethod]));
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

        let contructor_attribute: syn::Attribute = parse_quote!(#[capybara(constructor)]);

        let attribute_pos = rust_new
            .attrs
            .iter()
            .position(|x| &contructor_attribute == x);

        match attribute_pos {
            None => panic!("A constructor must have a #[capybara(constructor)] annotation"),
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

        let pyo3_new: syn::ImplItem = parse_quote!(
            #[new]
            fn __new__(obj: &PyRawObject, #(#args_decl,)*) -> PyResult<()> {
                obj.init(|| {
                    #classname::new(#(#args_usage,)*)
                })
            }
        );

        Some((syn::ImplItem::Method(rust_new), pyo3_new, rust_new_pos))
    }
}
