

use pyo3::prelude::*;
 
//use pyo3::type_gen::generate_type;
//use pyo3::type_gen::generate_type_as_function;
use crate::py_abstractions::structs::Objects::Two_D_Object::Two_D_Object;

use pyo3_stub_gen::derive::* ;



#[gen_stub_pyclass]
#[pyclass(extends = Two_D_Object)]
pub struct Circle{
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    
}
