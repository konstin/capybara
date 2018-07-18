#![feature(
    use_extern_macros,
    specialization,
    wasm_import_module,
    wasm_custom_section,
    concat_idents
)]

#[macro_use]
extern crate capybara;

use capybara::prelude::*;

#[capybara]
pub fn double(x: usize) -> usize {
    x * 2
}

#[capybara]
pub struct ExportedClass {
    number: i32,
}

#[capybara]
impl ExportedClass {
    #[capybara(constructor)]
    pub fn new(number: i32) -> ExportedClass {
        ExportedClass { number }
    }

    pub fn no_args() {}
    pub fn one_arg(_a: i32) {}
    pub fn two_args(_a: i32, _b: i32) {}

    pub fn no_args_returning() -> i32 {
        42
    }
    pub fn one_arg_returning(_a: i32) -> i32 {
        42
    }
    pub fn two_args_returning(_a: i32, _b: i32) -> i32 {
        42
    }

    pub fn self_no_args(&self) {}
    pub fn self_one_arg(&self, _a: i32) {}
    pub fn self_two_args(&self, _a: i32, _b: i32) {}

    pub fn self_no_args_returning(&self) -> i32 {
        42
    }
    pub fn self_one_arg_returning(&self, _a: i32) -> i32 {
        42
    }
    pub fn self_two_args_returning(&self, _a: i32, _b: i32) -> i32 {
        42
    }

    pub fn mut_self_no_args(&mut self) {}
    pub fn mut_self_one_arg(&mut self, _a: i32) {}
    pub fn mut_self_two_args(&mut self, _a: i32, _b: i32) {}

    pub fn mut_self_no_args_returning(&mut self) -> i32 {
        42
    }
    pub fn mut_self_one_arg_returning(&mut self, _a: i32) -> i32 {
        42
    }
    pub fn mut_self_two_args_returning(&mut self, _a: i32, _b: i32) -> i32 {
        42
    }

    pub fn add_and_print(x: i32, y: i32) -> i32 {
        println!("Printing from rust: {}", x + y);
        x + y
    }

    pub fn get_number(&self) -> i32 {
        self.number
    }
}

capybara_init! {capybara_test, [ExportedClass], [double]}
