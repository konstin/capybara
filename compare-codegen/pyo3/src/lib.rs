#![feature(specialization)]

use pyo3::prelude::*;

#[pyclass]
pub struct ExportedClass {
    number: i32,
}

#[pymethods]
impl ExportedClass {
    #[new]
    fn __new__(obj: &pyo3::type_object::PyRawObject, number: i32) {
        obj.init(ExportedClass { number })
    }

    #[staticmethod]
    pub fn no_args() -> () {}

    #[staticmethod]
    pub fn one_arg(_a: i32) -> () {}

    #[staticmethod]
    pub fn no_args_returning() -> i32 {
        42
    }

    #[staticmethod]
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

#[pymodule]
fn capybara_test(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ExportedClass>().unwrap();
    Ok(())
}
