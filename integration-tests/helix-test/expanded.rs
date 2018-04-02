#![feature(prelude_import)]
#![no_std]
#![feature(proc_macro, proc_macro_lib, specialization, const_fn)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;

#[macro_use]
extern crate omni;

use omni::{class, methods};
use omni::prelude::*;




use ::helix::*;
#[repr(C)]
struct MyClass {
    helix: ::Metadata,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for MyClass {
    #[inline]
    fn clone(&self) -> MyClass {
        match *self {
            MyClass { helix: ref __self_0_0 } =>
            MyClass{helix: ::std::clone::Clone::clone(&(*__self_0_0)),},
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for MyClass {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            MyClass { helix: ref __self_0_0 } => {
                let mut __debug_trait_builder =
                    __arg_0.debug_struct("MyClass");
                let _ = __debug_trait_builder.field("helix", &&(*__self_0_0));
                __debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals)]
static mut MyClass: usize = 0;
impl MyClass {
    fn print_and_double(x: i32) -> i32 {
        ::io::_print(::std::fmt::Arguments::new_v1_formatted(&["Printing from rust: ",
                                                               "\n"],
                                                             &match (&(x *
                                                                           2),)
                                                                  {
                                                                  (__arg0,) =>
                                                                  [::std::fmt::ArgumentV1::new(__arg0,
                                                                                               ::std::fmt::Display::fmt)],
                                                              },
                                                             &[::std::fmt::rt::v1::Argument{position:
                                                                                                ::std::fmt::rt::v1::Position::At(0usize),
                                                                                            format:
                                                                                                ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                   ' ',
                                                                                                                               align:
                                                                                                                                   ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                               flags:
                                                                                                                                   0u32,
                                                                                                                               precision:
                                                                                                                                   ::std::fmt::rt::v1::Count::Implied,
                                                                                                                               width:
                                                                                                                                   ::std::fmt::rt::v1::Count::Implied,},}]));
        x * 2
    }
}
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_native() {
    use ::InitRuby;
    ::sys::check_version();
    MyClass::init_ruby();
}
