#![feature(proc_macro, specialization, const_fn)]

#[macro_use]
extern crate capybara;

use capybara::capybara_bindgen;

#[capybara_bindgen]
pub struct MyClass {
    number: i32,
}

#[capybara_bindgen]
impl MyClass {
    pub fn new(number: i32) -> MyClass {
        MyClass { number }
    }

    pub fn add_and_print(x: i32, y: i32) -> i32 {
        println!("Printing from rust: {}", x + y);
        x + y
    }

    pub fn get_number(&self) -> i32 {
        self.number
    }
}

capybara_init! {pyo3_test, [MyClass]}
