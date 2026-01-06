use pyo3::prelude::*;
use macroquad::prelude as mq;

use pyo3_stub_gen::derive::* ;
use slotmap::DefaultKey;
use slotmap::Key;
use crate::py_abstractions::structs::GLAM::Vec3::Vec3;
use crate::engine::Objects::Cube as cu;
use crate::engine::Objects::ObjectManagement::ObjectStorage as oj;
use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;
use std::sync::mpsc;
use pyo3::types::{PyWeakref, PyWeakrefReference};
use crate::py_abstractions::Color::Color;



#[gen_stub_pyclass]
#[pyclass(subclass, weakref)]
#[derive(Clone)]
pub struct Cube{
    key: DefaultKey, // the key to the actual underlying cube, stored inside "ObjectStorage"
}

#[gen_stub_pymethods]
#[pymethods]
impl Cube {

    #[pyo3(signature = (position, size, rotation = Vec3::ZERO(), color = Color::WHITE()   ))]
    #[new]
    pub fn new(
        py: Python<'_>,
        position: Vec3,
        size: Vec3,
        rotation: Vec3,
        color: Color,
    ) -> PyResult<Py<Cube>> {

        let (sender, receiver) = mpsc::sync_channel(1);

        let placeholder_struct: Cube = Cube { key: DefaultKey::null() };
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
            color: color.into(),
            pyAny: weak_ref_handle,
            sender 
        });
        
        let key: DefaultKey = receiver.recv().unwrap();
        
        cube_handle.borrow_mut(py).key = key;
        Ok(cube_handle) 

        
    }
        


    #[getter]
    fn size(&self) -> Vec3 {
        let (sender, receiver) = mpsc::sync_channel(1);
        let command = Command::GetCubeSize { key: self.key, sender: sender };
        COMMAND_QUEUE.push(command);
        receiver.recv().unwrap().into()
    }

    #[setter]
    fn set_size(&self, value: Vec3) {
        let command = Command::SetCubeSize { key: self.key, size: value.into() };
        COMMAND_QUEUE.push(command);
    }

    #[getter]
    fn pos(&self) -> Vec3 {
        let (sender, receiver) = mpsc::sync_channel(1);
        let command = Command::GetCubePos { key: self.key, sender: sender };
        COMMAND_QUEUE.push(command);
        receiver.recv().unwrap().into()
    }

    #[setter]
    fn set_pos(&self, value: Vec3) {
        let command = Command::SetCubePos { key: self.key, position: value.into() };
        COMMAND_QUEUE.push(command);
    }

    #[getter]
    fn rot(&self) -> Vec3 {
        let (sender, receiver) = mpsc::sync_channel(1);
        let command = Command::GetCubeRotation{ key: self.key, sender: sender };
        COMMAND_QUEUE.push(command);
        receiver.recv().unwrap().into()
    }

    #[setter]
    fn set_rot(&mut self, value: Vec3) {
        let command = Command::SetCubeRotation { key: self.key, rotation: value.into() };
        COMMAND_QUEUE.push(command);
    }
    
}

impl Drop for Cube{
    fn drop(&mut self) {
        COMMAND_QUEUE.push( Command::deleteCube { key: self.key });
    }
}