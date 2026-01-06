use pyo3::prelude::*;
use crate::py_abstractions::structs::Objects::Two_D_Object::TwoDObject;
use pyo3_stub_gen::derive::* ;



#[gen_stub_pyclass]
#[pyclass(extends = TwoDObject)]
pub struct Circle{
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    
}
impl Circle{
    pub fn new(){
        todo!()
    }
}
