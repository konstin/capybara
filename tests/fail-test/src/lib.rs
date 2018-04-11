#![feature(proc_macro, specialization, const_fn)]

#[macro_use]
extern crate capybara;

use capybara::capybara_bindgen;

#[capybara_bindgen]
pub struct ExportedClass {}

#[capybara_bindgen]
impl ExportedClass {
    pub fn add_and_print(x: i32, y: i32) -> i32 {
        println!("Printing from rust: {}", x + y);
        x + y
    }
}

capybara_init! {fail_test, [ExportedClass]}
