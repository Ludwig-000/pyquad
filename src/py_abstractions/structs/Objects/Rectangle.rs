use pyo3::prelude::*;
use pyo3_stub_gen::derive::* ;


#[gen_stub_pyclass]
#[pyclass(subclass)]
#[derive(Clone,Copy, PartialEq)]
pub struct Rectangle{
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[gen_stub_pymethods]
#[pymethods]
impl Rectangle {

    #[new]
    pub fn new()-> Self{
        todo!()
    }
}