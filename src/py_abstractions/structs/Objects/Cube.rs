use pyo3::prelude::*;
use macroquad::prelude as mq;

use pyo3_stub_gen::derive::* ;
use crate::py_abstractions::structs::GLAM::Vec3::Vec3;


#[gen_stub_pyclass]
#[pyclass(subclass)]
#[derive(Clone)]
pub struct Cube{
    engine_cube: crate::engine::Objects::Cube::Cube,
}

#[gen_stub_pymethods]
#[pymethods]
impl Cube {

    #[new]
    pub fn new()-> Self{
        todo!()
    }


    #[getter]
    fn size(&self) -> Vec3 {
        todo!()
    }

    #[setter]
    fn set_size(&mut self, value: Vec3) {
        todo!()
    }

    #[getter]
    fn pos(&self) -> Vec3 {
        todo!()
    }

    #[setter]
    fn set_pos(&mut self, value: Vec3) {
        todo!()
    }
    
}