#![feature(specialization)]

#[macro_use]
extern crate capybara;

use capybara::prelude::*;

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

capybara_init! {capybara_test, [ExportedClass], []}
