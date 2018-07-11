#![feature(proc_macro, proc_macro_lib, specialization, external_doc)]
#![doc(include = "../Readme.md")]

extern crate capybara_derive;
#[cfg(feature = "ruby")]
pub extern crate helix;
#[cfg(feature = "python")]
pub extern crate pyo3;
#[cfg(feature = "wasm")]
pub extern crate wasm_bindgen;

pub use capybara_derive::capybara_bindgen;

/// This is a workaround for bringing some items for the proc macro in scope. Use with
/// `use capybara::prelude::*`
///
/// Importing in the output of the proc macro isn't allowed for the top level and doesn't work
/// inside a constant (probably a hygiene bug). A complex and yet-to-be-implemented workaround
/// would be to reqrite the paths in the output of the used libraries' proc macros-
pub mod prelude {
    pub use capybara_derive::capybara_bindgen as capybara;

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

/// Exports your classes and functions to the runtime
///
/// The first parameter is the name of the module, the second a list of the names of the classes
/// to export and the third is a list of the functions to export.
///
/// For ruby, the name of the module is irrelevant (it will always create a function called
/// Init_native), but for python the module name must match the name of the imported shared object,
/// i.e. when the module is caleed my_module, the generated file must be renamed to my_module.so
/// or to my_module.pyd on windows. (pyo3 generates a function called PyInit_my_module).
///
/// Note that this is only a stub, the real implementations for python and ruby are selected by a
/// feature-gate.
///
/// # Example
///
/// ```
/// # #![feature(proc_macro, specialization, wasm_import_module, wasm_custom_section, concat_idents)]
/// # use capybara::*;
/// # use capybara::prelude::*;
/// #[capybara]
/// fn say_hello() {
///    println!("Hello world");
/// }
/// #[capybara]
/// pub struct ExportedClass {}
/// capybara_init! {my_module, [ExportedClass], [say_hello]}
/// ```
#[cfg(not(any(feature = "python", feature = "ruby")))]
#[macro_export]
macro_rules! capybara_init {
    ($modname:ident,[$($class:ident),*],[$($function:ident),*]) => {};
}

#[cfg(feature = "python")]
#[macro_export]
macro_rules! capybara_init {
    ($modname:ident, [$( $class:ident ),*], [$( $function:ident ),*]) => {
        use $crate::pyo3;
        use $crate::pyo3::pymodinit;
        #[pymodinit($modname)]
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

/// This macro is doing essentially the same as helix' parse! macro with state: parse_struct, i.e.
/// parsing the struct and forwarding it to codegen_struct!.
#[cfg(feature = "ruby")]
#[macro_export]
macro_rules! codegen_from_struct {
    (
        pub struct $name:ident { $($struct:tt)* }
    ) => {
        // Get the macros into scope
        codegen_struct! {
            pub: true,
            rust_name: $name,
            ruby_name: { stringify!($name) },
            struct: { $($struct)* }
        }
    };
}
