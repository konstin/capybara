#![feature(proc_macro, specialization, const_fn)]

#[macro_use]
extern crate omni;

use omni::prelude::*;
use omni::{class, methods};

#[class]
struct MyClass {}

#[methods]
impl MyClass {
    fn print_and_double(x: i32) -> i32 {
        println!("Printing from rust: {}", x * 2);
        x * 2
    }
}

omni_init! {pyo3_test, [MyClass]}
