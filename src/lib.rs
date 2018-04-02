#![feature(proc_macro, proc_macro_lib, specialization, const_fn)]

pub extern crate helix;
extern crate omni_derive;
pub extern crate pyo3;
extern crate pyo3cls;

pub use omni_derive::{class, methods};

#[cfg(feature = "use_pyo3")]
pub mod prelude {
    pub use pyo3;
    pub use pyo3cls::mod3init as pyo3_init;
    pub use pyo3::prelude::PyResult;
}

#[cfg(feature = "use_helix")]
pub mod prelude {}


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


#[macro_export]
macro_rules! codegen_from_struct {
    {
        struct $name:ident { $($struct:tt)* }
    } => {
        // Get the macros into the scop
        use $crate::helix::*;
        codegen_struct! {
            pub: false,
            rust_name: $name,
            ruby_name: { stringify!($name) },
            struct: { $($struct)* }
        }
    }
}