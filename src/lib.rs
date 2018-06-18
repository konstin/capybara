#![feature(proc_macro, proc_macro_lib, specialization)]

extern crate capybara_derive;
#[cfg(feature = "ruby")]
pub extern crate helix;
#[cfg(feature = "python")]
pub extern crate pyo3;
#[cfg(feature = "python")]
pub extern crate pyo3cls;
#[cfg(feature = "wasm")]
pub extern crate wasm_bindgen;

pub use capybara_derive::capybara_bindgen;

/// It's not allowed to import the same item twice, so we use this module with a star import instead
pub mod prelude {
    #[cfg(feature = "wasm")]
    pub use wasm_bindgen;
    #[cfg(feature = "wasm")]
    pub use wasm_bindgen::prelude::*;

    #[cfg(feature = "ruby")]
    pub use helix;

    #[cfg(feature = "ruby")]
    pub use helix::*;

    #[cfg(feature = "python")]
    pub use pyo3;

    #[cfg(feature = "python")]
    pub use pyo3::prelude::*;
}

#[cfg(all(not(target_arch = "wasm32"), feature = "wasm"))]
compile_error!("You need to pass --target wasm32-unknown-unknown to compile to wasm");

/// Creates the FFI entrypoint.
///
/// The first parameter is the name of the module, the second the names of the classes to export.
/// For helix, the name of the module is irrelevant (it will always create a function called
/// Init_native), but for pyo3 it must match the name of the imported shared object, i.e. for
/// my_module on linux the generated file must be renamed my_module.so. (pyo3 generates a function
/// called PyInit_<modname>).
///
/// N.B. This is only a stub, the real implementation is selected by a feature-gate.
///
/// # Example
///
/// ```
/// #[capybara_bindgen]
/// pub struct ExportedClass {}
/// capybara_init! (my_module, [ExportedClass]);
/// ```
#[macro_export]
macro_rules! capybara_init {
    () => {};
}

#[cfg(feature = "python")]
#[macro_export]
macro_rules! capybara_init {
    ($modname:ident, [$( $class:ident ),*], [$( $function:ident ),*]) => {
        use $crate::pyo3;
        use $crate::pyo3cls::mod3init as pyo3_init;
        #[pyo3_init($modname)]
        fn capybara_init(_py: $crate::pyo3::Python, m: &$crate::pyo3::PyModule) -> $crate::pyo3::PyResult<()> {
            $(
                m.add_class::<$class>().unwrap();
            )*
            $(
                m.add_function($crate::pyo3::wrap_function!($function)).unwrap();
            )*
            Ok(())
        }
    };
}

#[cfg(feature = "ruby")]
#[macro_export]
macro_rules! capybara_init {
    ($modname:ident, [$( $class:ident ),*], [$( $function:ident ),*]) => {
        codegen_init!([$( $class ),*]);
    };
}

#[cfg(not(any(feature = "python", feature = "ruby")))]
#[macro_export]
macro_rules! capybara_init {
    ($modname:ident,[$($class:ident),*],[$($function:ident),*]) => {};
}

/// This macro is doing essentially the same as helix' parse! macro with state: parse_struct, i.e.
/// parsing the struct and forwarding it to codegen_struct!.
///
/// The only catch here is that the other helix macros (and especially codegen_struct!) are defined
/// in the helix crate, so we need to get them into scope. The current use $crate::helix::*; works,
/// though there's surely something more elegant.
#[cfg(feature = "ruby")]
#[macro_export]
macro_rules! codegen_from_struct {
    {
        struct $name:ident { $($struct:tt)* }
    } => {
        // Get the macros into scope
        codegen_struct! {
            pub: false,
            rust_name: $name,
            ruby_name: { stringify!($name) },
            struct: { $($struct)* }
        }
    };

    {
        pub struct $name:ident { $($struct:tt)* }
    } => {
        // Get the macros into scope
        codegen_struct! {
            pub: true,
            rust_name: $name,
            ruby_name: { stringify!($name) },
            struct: { $($struct)* }
        }
    };
}
