use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;

use pyo3_stub_gen::derive::* ;
use slotmap::DefaultKey;
use slotmap::Key;
use crate::engine::Objects::ObjectDataCache;
use crate::py_abstractions::structs::GLAM::Vec3::Vec3;
use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;
use crate::py_abstractions::structs::Objects::ObjectFunctionStorage;
use std::fmt::Pointer;
use std::sync::mpsc;
use pyo3::types::{PyWeakref, PyWeakrefReference};
use crate::py_abstractions::Color::Color;
use pyo3::exceptions::*;

#[gen_stub_pyclass]
#[pyclass(subclass, weakref)]
#[derive(Clone)]
pub struct Cube{
    key: DefaultKey, // the key to the actual underlying cube, stored inside "ObjectStorage"


    /// we add a cache for trivial data, which can be used if the object is not
    /// influenced by outside forces F.E. Gravity.
    cache: ObjectDataCache::ThreeDObjCache,
}

#[gen_stub_pymethods]
#[pymethods]
impl Cube {

    #[pyo3(signature = (position= Vec3::ZERO(), rotation = Vec3::ZERO(),scale= Vec3::ONE(), color = Color::WHITE()))]
    #[new]
    pub fn new(
        py: Python<'_>,
        position: Vec3,
        rotation: Vec3,
        scale: Vec3,
        color: Color,
    ) -> PyResult<Py<Cube>> {

        let (sender, receiver) = mpsc::sync_channel(1);

        let cache = ObjectDataCache::ThreeDObjCache::new(true, position.into(), rotation.into(), scale.into(), color.into());
        let placeholder_struct: Cube = Cube { key: DefaultKey::null(),  cache};
        let cube_handle: Py<Cube> = Py::new(py, placeholder_struct)?; 
        
        let weak_ref_handle: Py<PyWeakref> = {
            let bound_cube = cube_handle.bind(py); 
            let weak_ref_ref = PyWeakrefReference::new(&bound_cube)?;
            
            weak_ref_ref.cast_into::<PyWeakref>()?.unbind() 
        };

        
        COMMAND_QUEUE.push(Command::CreateCube { 
            size: scale.into(), 
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
    fn scale(&self) -> Vec3 {
        if self.cache.can_be_cached == true{
            return self.cache.scale.into()
        }

        let (sender, receiver) = mpsc::sync_channel(1);
        let command = Command::GetCubeSize { key: self.key, sender: sender };
        COMMAND_QUEUE.push(command);
        receiver.recv().unwrap().into()
    }

    #[setter]
    fn set_scale(&mut self, value: Vec3) {
        if self.cache.can_be_cached == true{
            self.cache.scale = value.into();
        }

        let command = Command::SetCubeSize { key: self.key, size: value.into() };
        COMMAND_QUEUE.push(command);
    }

    #[getter]
    fn pos(&self) -> Vec3 {
        if self.cache.can_be_cached == true{
            return self.cache.location.into()
        }
        let (sender, receiver) = mpsc::sync_channel(1);
        let command = Command::GetCubePos { key: self.key, sender: sender };
        COMMAND_QUEUE.push(command);
        receiver.recv().unwrap().into()
    }

    #[setter]
    fn set_pos(&mut self, value: Vec3) {
        if self.cache.can_be_cached == true{
            self.cache.location = value.into();
        }
        let command = Command::SetCubePos { key: self.key, position: value.into() };
        COMMAND_QUEUE.push(command);
    }

    #[getter]
    fn rot(&self) -> Vec3 {
        if self.cache.can_be_cached == true{
            return self.cache.rotation.into()
        }

        let (sender, receiver) = mpsc::sync_channel(1);
        let command = Command::GetCubeRotation{ key: self.key, sender: sender };
        COMMAND_QUEUE.push(command);
        receiver.recv().unwrap().into()
    }

    #[setter]
    fn set_rot(&mut self, value: Vec3) {
        if self.cache.can_be_cached == true{
            self.cache.rotation = value.into();
        }

        let command = Command::SetCubeRotation { key: self.key, rotation: value.into() };
        COMMAND_QUEUE.push(command);
    }


    pub fn check_collision<'py>(&self, py: Python<'py> )-> Vec<Bound<'py, PyAny>>{

        let (sender, receiver) = mpsc::sync_channel(1);
        let command = Command::GetColissionObjects { key: self.key, sender };
        COMMAND_QUEUE.push(command);
        let res: Vec<std::sync::Arc<Py<PyWeakref>>> = receiver.recv().unwrap();


        res.into_iter().filter_map(|pyObj|{

            let weak_py: &Py<PyWeakref> = &*pyObj;

            let weak_bound: Bound<'py, PyWeakref> = weak_py.bind(py).clone();

            let upgraded: Bound<'py, PyAny> = weak_bound.call0().ok()?;

            if upgraded.is_none() {
                None
            } else {
                Some(upgraded)
            }
        }).collect()
    }

    /// Add a function to this object, which will automatically be executed each frame.
    /// The function may take 'self' as the first argument.
    /// 
    /// example:
    /// 
    /// 
    /// def fun(cube):
    ///     cube.x+=1
    /// 
    /// myCube.tick(fun)
    /// 
    pub fn tick(slf: Bound<'_, Self>, function: Bound<'_,PyAny>)-> PyResult<()>{

        if !function.is_callable(){
            return Err(PyRuntimeError::new_err(format!("Attatched object {:?} is not callable.",function)));
        }

        let mut storage = ObjectFunctionStorage::get_fun_storage();
        
        let func_persistent = function.unbind();
        let obj  = slf.into_any();

        storage.add(obj, func_persistent);

        Ok(())
    }

    
}

impl Drop for Cube{
    fn drop(&mut self) {
        COMMAND_QUEUE.push( Command::DeleteCube { key: self.key });
    }
}