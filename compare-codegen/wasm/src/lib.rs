#![feature(proc_macro, wasm_import_module, wasm_custom_section)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ExportedClass {
    number: i32,
}

#[wasm_bindgen]
impl ExportedClass {
    #[wasm_bindgen(constructor)]
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
