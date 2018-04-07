#![feature(proc_macro, specialization, const_fn)]
#![feature(wasm_import_module, wasm_custom_section)]

#[macro_use]
extern crate capybara;

use capybara::capybara_bindgen;

#[capybara_bindgen]
pub struct MyClass {}

#[capybara_bindgen]
impl MyClass {
    pub fn add_and_print(x: i32, y: i32) -> i32 {
        println!("Printing from rust: {}", x + y);
        x + y
    }
}

capybara_init! {wasm_test, [MyClass]}
