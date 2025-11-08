

use pyo3::prelude::*;
 
//use pyo3::type_gen::generate_type;
//use pyo3::type_gen::generate_type_as_function;

use pyo3_stub_gen::derive::* ;


#[gen_stub_pyclass]
#[pyclass(subclass)]
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
        Rectangle { x: 9.1, y: 9.1, width: 9.1, height: 9.1 }
    }
}