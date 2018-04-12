#![feature(proc_macro, specialization, wasm_import_module, wasm_custom_section)]

#[macro_use]
extern crate capybara;

use capybara::capybara_bindgen;

#[capybara_bindgen]
pub struct ExportedClass {
    number: i32,
}

#[capybara_bindgen]
impl ExportedClass {
    pub fn new(number: i32) -> ExportedClass {
        ExportedClass { number }
    }

    pub fn no_args() {}
    pub fn one_arg(_a: i32) {}
    pub fn no_args_returning() -> i32 {
        42
    }
    pub fn one_arg_returning(_a: i32) -> i32 {
        42
    }

    pub fn self_no_args_returning(&self) -> i32 {
        42
    }

    pub fn self_mut_no_args_returning(&mut self) -> i32 {
        42
    }

    pub fn get_number(&self) -> i32 {
        self.number
    }
}

capybara_init! {capybara_test, [ExportedClass]}
