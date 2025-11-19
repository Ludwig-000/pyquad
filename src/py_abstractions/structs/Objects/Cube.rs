use pyo3::prelude::*;
use macroquad::prelude as mq;

use pyo3_stub_gen::derive::* ;
use crate::py_abstractions::structs::GLAM::Vec3::Vec3;
use crate::engine::Objects::Cube as cu;
use crate::engine::Objects::ObjectManagement::ObjectStorage as oj;
use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;
use std::sync::mpsc;
use pyo3::types::{PyWeakref, PyWeakrefReference};

#[gen_stub_pyclass]
#[pyclass(subclass, weakref)]
#[derive(Clone)]
pub struct Cube{
    key: usize, // the key to the vec. the actual vec is static, so we do not need to link to it.
    // don't worry about memory safety.
}

#[gen_stub_pymethods]
#[pymethods]
impl Cube {

    #[pyo3(signature = (size, position, rotation = Vec3::ZERO()))]
    #[new]
    pub fn new(
        py: Python<'_>,
        size: Vec3,
        position: Vec3,
        rotation: Vec3,
    ) -> PyResult<Py<Cube>> {

        let (sender, receiver) = mpsc::sync_channel(1);

        let placeholder_struct: Cube = Cube { key: 0 };
        let cube_handle: Py<Cube> = Py::new(py, placeholder_struct)?; 
        
        let weak_ref_handle: Py<PyWeakref> = {
            let bound_cube = cube_handle.bind(py); 
            let weak_ref_ref = PyWeakrefReference::new(&bound_cube)?;
            
            weak_ref_ref.cast_into::<PyWeakref>()?.unbind() 
        };

        
        COMMAND_QUEUE.push(Command::createCube { 
            size: size.into(), 
            position: position.into(), 
            rotation: rotation.into(), 
            pyAny: weak_ref_handle,
            sender 
        });
        
        let key: usize = receiver.recv().unwrap();
        
        cube_handle.borrow_mut(py).key = key;
        Ok(cube_handle) 

        
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

