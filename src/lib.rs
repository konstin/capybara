#![feature(proc_macro, proc_macro_lib, specialization, const_fn)]

pub extern crate helix;
extern crate omni_derive;
pub extern crate pyo3;
extern crate pyo3cls;

pub use omni_derive::{class, methods};

#[cfg(feature = "use_pyo3")]
pub mod prelude {
    pub use pyo3;
    pub use pyo3::prelude::PyResult;
    pub use pyo3cls::mod3init as pyo3_init;
}

#[cfg(feature = "use_helix")]
pub mod prelude {}

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
/// #[class]
/// #struct MyClass {}
/// omni_init! (my_module, [MyClass]);
/// ```
///
#[macro_export]
macro_rules! omni_init {
    () => {};
}

#[cfg(feature = "use_pyo3")]
#[macro_export]
macro_rules! omni_init {
    ( $modname:ident, [$( $classname:ty ),*] ) => {
        use $crate::pyo3::prelude::*;
        #[$crate::prelude::pyo3_init($modname)]
        fn omni_init(_py: Python, m: &PyModule) -> PyResult<()> {
            $(
                m.add_class::<$classname>().unwrap();
            )*
            Ok(())
        }
    };
}

#[cfg(feature = "use_helix")]
#[macro_export]
macro_rules! omni_init {
     { $modname:ident, [$( $classname:ident ),*] } => {
        codegen_init!([$( $classname ),*]);
    }
}

/// This macro is doing essentially the same as helix' parse! macro with state: parse_struct, i.e.
/// parsing the struct and forwarding it to codegen_struct!.
///
/// The only catch here is that the other helix macros (and especially codegen_struct!) are defined
/// in the helix crate, so we need to get them into scope. The current use $crate::helix::*; works,
/// though there's surely something more elegant.
#[cfg(feature = "use_helix")]
#[macro_export]
macro_rules! codegen_from_struct {
    {
        struct $name:ident { $($struct:tt)* }
    } => {
        // Get the macros into scope
        use $crate::helix::*;
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
        use $crate::helix::*;
        codegen_struct! {
            pub: true,
            rust_name: $name,
            ruby_name: { stringify!($name) },
            struct: { $($struct)* }
        }
    };
}
