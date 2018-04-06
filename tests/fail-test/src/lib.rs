#![feature(proc_macro, specialization, const_fn)]

#[macro_use]
extern crate omni;

use omni::omni_bindgen;

#[omni_bindgen]
struct MyClass {}

#[omni_bindgen]
impl MyClass {
    fn add_and_print(x: i32, y: i32) -> i32 {
        println!("Printing from rust: {}", x + y);
        x + y
    }
}

omni_init! {fail_test, [MyClass]}
