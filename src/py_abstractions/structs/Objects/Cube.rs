

use pyo3::prelude::*;
 
//use pyo3::type_gen::generate_type;
//use pyo3::type_gen::generate_type_as_function;
use macroquad::prelude as mq;

use pyo3_stub_gen::derive::* ;
use crate::py_abstractions::structs::GLAM::Vec3::Vec3;

#[derive(Clone)]
pub struct internal_Cube{
    pub size: mq::Vec3,
    pub pos: mq::Vec3,
}

#[gen_stub_pyclass]
#[pyclass(subclass)]
#[derive(Clone)]
pub struct Cube{
    pub inner: Box<internal_Cube>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Cube {

    #[new]
    pub fn new()-> Self{
        Cube { inner: Box::new( internal_Cube{  size: mq::Vec3::default(), pos: mq::Vec3::default()  }  ) }
    }


    #[getter]
    fn size(&self) -> Vec3 {
        self.inner.size.into()
    }

    #[setter]
    fn set_size(&mut self, value: Vec3) {
        self.inner.size = value.into();
    }

    #[getter]
    fn pos(&self) -> Vec3 {
        self.inner.pos.into()
    }

    #[setter]
    fn set_pos(&mut self, value: Vec3) {
        self.inner.pos = value.into();
    }
    
}