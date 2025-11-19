use pyo3::prelude::*;
use macroquad::prelude as mq;

use pyo3_stub_gen::derive::* ;
use crate::py_abstractions::structs::GLAM::Vec3::Vec3;
use crate::engine::Objects::Cube as cu;
use crate::engine::Objects::ObjectManagement::ObjectStorage as oj;
use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;
use std::sync::mpsc;

#[gen_stub_pyclass]
#[pyclass(subclass)]
#[derive(Clone)]
pub struct Cube{
    key: usize, // the key to the vec. the actual vec is static, so we do not need to link to it.
    // don't worry about memory safety.
}
use pyo3::types::{PyWeakref, PyWeakrefReference};
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
        let bound_obj: Bound<'_, Cube> = pyo3::Bound::new(py, placeholder_struct)?;
        let pyAny: pyo3::Py<pyo3::PyAny> = bound_obj.clone().unbind().into();


        // let cube_handle: Py<Cube> = Py::new(py, placeholder_struct)?;
        
        // // 2. Prepare the Weak Reference from the strong reference.
        // let weak_ref_handle: Py<PyWeakref> = {
        //     // Bind the Py<Cube> handle to get a Bound<Cube> reference.
        //     let bound_obj = cube_handle.bind(py); 
            
        //     // Create the PyWeakrefReference (which knows how to make the weak ref).
        //     let weak_ref_ref = PyWeakrefReference::new(&bound_obj)?;
            
        //     // Cast to the persistent Py<PyWeakref> handle and unbind it.
        //     // This is the persistent, Send-safe handle you need.
        //     weak_ref_ref.cast_into::<PyWeakref>()?.unbind()
        // };

        println!("One");
        COMMAND_QUEUE.push(Command::createCube { size: size.into(), position: position.into(), rotation: rotation.into(), pyAny, sender });
        match receiver.recv().unwrap() {
            
            Ok(key) => {
                println!("SIX");
                bound_obj.borrow_mut().key = key;

                Ok(bound_obj.unbind())
            }
            Err(e) => {
                println!("SIX");
                Err(e.into())
            }
    }
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

impl Drop for Cube {
    fn drop(&mut self) {

        todo!()
    }
}