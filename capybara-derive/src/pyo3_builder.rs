//! Beware, lands of ugly syn 0.11 code wrapping an ugly library lie ahead of you

extern crate pyo3;
extern crate pyo3_derive_backend;
extern crate syn_0_11;

use self::syn_0_11::*;
use super::BindingBuilder;
use proc_macro::TokenStream;
use std::str::FromStr;

pub struct Pyo3Builder;

/// This is the same boilerplate that pyo3 uses
impl BindingBuilder for Pyo3Builder {
    fn class(&self, attr: TokenStream, input: TokenStream) -> TokenStream {
        let mut ast = parse_derive_input(&input.to_string()).unwrap();
        let expanded = pyo3_derive_backend::py_class::build_py_class(&mut ast, attr.to_string());
        let tokens = quote!(
            #ast
            #expanded
        );

        TokenStream::from_str(tokens.as_str()).unwrap()
    }

    fn methods(&self, _: TokenStream, input: TokenStream) -> TokenStream {
        let mut ast = parse_item(&input.to_string()).unwrap();

        let expanded = if let ItemKind::Impl(_, _, _, None, ref ty, ref mut methods) = ast.node {
            let classname = ty.clone();
            let constructor = Pyo3Builder::constructor(methods, &classname);
            Pyo3Builder::add_function_annotations(methods);

            let rust_new = if let Some((rust_new, pyo3_new, rust_new_pos)) = constructor {
                methods.insert(rust_new_pos, pyo3_new);

                Some(rust_new)
            } else {
                None
            };

            let expanded = pyo3_derive_backend::py_impl::impl_methods(ty, methods);

            // Add the initial new method back
            let expanded = if let Some(new_method_impl) = rust_new {
                quote!(
                    #expanded

                    impl #classname {
                        #new_method_impl
                    }
                )
            } else {
                expanded
            };
            expanded
        } else {
            panic!("Expected an impl block");
        };

        let tokens = quote!(
            #ast
            #expanded
        );

        TokenStream::from_str(tokens.as_str()).unwrap()
    }

    fn foreign_mod(&self, _: TokenStream, _: TokenStream) -> TokenStream {
        unimplemented!()
    }
}

#[cfg(feature = "python")]
impl Pyo3Builder {
    /// pyo3 expects static methods to be annotated, so let's add that annotation on static methods.
    fn add_function_annotations(methods: &mut Vec<ImplItem>) {
        for method in methods.iter_mut() {
            let is_static;

            if let ImplItemKind::Method(ref mut signature, _) = method.node {
                is_static = match signature.decl.inputs.get(0) {
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
    }

    /// Builds a pyo3 style constructor from a rust style one, given one does exist
    ///
    /// The rust new will be removed from the method list and returned together with the generated
    /// pyo3 one
    fn constructor(
        methods: &mut Vec<ImplItem>,
        classnamen: &Ty,
    ) -> Option<(ImplItem, ImplItem, usize)> {
        let rust_new_pos = methods
            .iter()
            .position(|method| method.ident == Ident::new("new"));

        let (rust_new, rust_new_pos) = match rust_new_pos {
            // pyo3 can't deal with that method, so we remove it
            Some(pos) => (methods.remove(pos), pos),
            None => return None,
        };

        let (signature, block) = match rust_new.clone().node {
            ImplItemKind::Method(signature, block) => (signature, block),
            _ => return None,
        };

        let new_expression = match block.stmts.last() {
            Some(Stmt::Expr(expr)) => expr,
            Some(_) => panic!("The last statement of a constructor must be an expression."),
            None => panic!("A constructor can not be empty"),
        };

        match new_expression.node {
            ExprKind::Struct(_, _, _) => {}
            _ => panic!("The last expression in a constructor must be instantiating a struct"),
        };

        let args: Vec<FnArg> = signature.decl.inputs.clone();
        let args2: Vec<Pat> = args.iter()
            .map(|x| match x {
                FnArg::Captured(pat, _) => pat.clone(),
                _ => panic!("Argument type not expected in constructor: {:?}", x),
            })
            .collect();

        // I've tried building this with syn primitives but the code became unmanagable,
        // so yes, I'm actually doing this with serializing and deserializing
        let pyo3_new = quote!(
            impl #classnamen {
                #[new]
                fn __new__(obj: &PyRawObject, #(#args,)*) -> PyResult<()> {
                    obj.init(|_| {
                        #classnamen::new(#(#args2,)*)
                    })
                }
            }
        ).to_string();

        let node = parse_item(&pyo3_new).unwrap().node;

        let pyo3_new = if let ItemKind::Impl(_, _, _, None, _, mut methods) = node {
            methods.pop().unwrap()
        } else {
            panic!();
        };

        Some((rust_new, pyo3_new, rust_new_pos))
    }
}
