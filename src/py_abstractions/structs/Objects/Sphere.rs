use pyo3::prelude::*;
use pyo3_stub_gen::derive::* ;
use pyo3::types::{PyWeakref, PyWeakrefReference};
use pyo3::exceptions::*;

use std::sync::mpsc;
use std::hash::{Hash, Hasher};

use slotmap::DefaultKey;
use slotmap::Key;

use crate::engine::Objects::ObjectDataCache;
use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;

use crate::py_abstractions::structs::GLAM::Vec3::Vec3;
use crate::py_abstractions::structs::Objects::ObjectFunctionStorage;
use crate::py_abstractions::Color::Color;


#[gen_stub_pyclass]
#[pyclass(subclass, weakref)]
pub struct Sphere{
    key: DefaultKey, // The key to the actual underlying cube, stored inside "ObjectStorage".

    // Key to a function inside 'function storage', which will be run each frame by the engine.
    function_key: Option<DefaultKey>,

    /// we add a cache for trivial data, which can be used if the object is not
    /// influenced by outside forces F.E. Gravity.
    cache: ObjectDataCache::ThreeDObjCache,
}

#[gen_stub_pymethods]
#[pymethods]
impl Sphere {

    #[pyo3(signature = (position= Vec3::ZERO(), rotation = Vec3::ZERO(),scale= Vec3::ONE(), color = Color::WHITE()))]
    #[new]
    pub fn new(
        py: Python<'_>,
        position: Vec3,
        rotation: Vec3,
        scale: Vec3,
        color: Color,
    ) -> PyResult<Py<Sphere>> {

        let (sender, receiver) = mpsc::sync_channel(1);

        let cache = ObjectDataCache::ThreeDObjCache::new(
            true, position.into(), rotation.into(), scale.into(), color.into());
            
        let placeholder_struct: Sphere = Sphere { key: DefaultKey::null(),function_key: None,  cache};
        let cube_handle: Py<Sphere> = Py::new(py, placeholder_struct)?; 
        
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

    pub fn disable_collision(&self){
        todo!()
    }
    pub fn enable_collision(&self){
        todo!()
    }

    /// Returns any object, with active collision, that is either
    /// intersected or inserted in the current object.
    /// 
    ///Example:
    /// 
    ///```
    ///>>>bigCube: Cube = Cube(pos=Vec3.splat(50))
    ///>>>intersected: list[Cube] = bigCube.check_collision()
    ///...
    ///...# since the returned objects are references, we can edit them directly
    ///...# without creating duplicates.
    ///>>>for cube in intersected:
    ///...   cube.pos = Vec3.ZERO()
    ///```
    pub fn check_collision<'py>(&self, py: Python<'py> )-> Vec<Bound<'py, PyAny>>{

        let (sender, receiver) = mpsc::sync_channel(1);
        let command = Command::GetColissionObjects { key: self.key, sender };
        COMMAND_QUEUE.push(command);
        let res: Vec<std::sync::Arc<Py<PyWeakref>>> = receiver.recv().unwrap();

        // we filter map, since any weakref we hold may already be invalid.
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
    /// The function must take the object it is attatched to as an argument.
    /// 
    ///Example:
    /// 
    ///```
    ///...# arguments from outside the scope may be included.
    ///>>>delta_time = 0
    ///>>>def updateCube(cube: Cube):
    ///...    cube.rot += Vec3.splat(0.2*delta_time)
    ///...
    ///>>>myCube = Cube()
    ///>>>myCube.tick(updateCube)
    ///...
    ///>>>while True:
    ///...    # dt would have to get updated each frame.
    ///...    delta_time = get_delta_time()
    ///...
    ///...    #'next_frame' runs the update function for every object.
    ///...    next_frame()
    /// ```
    pub fn tick(slf: Bound<'_, Self>, function: Bound<'_,PyAny>)-> PyResult<()>{

        if !function.is_callable(){
            return Err(PyRuntimeError::new_err(format!("Attatched object {:?} is not callable.",function)));
        }

        let mut storage = ObjectFunctionStorage::get_fun_storage();
        
        let func_persistent = function.unbind();
        let obj  = slf.into_any();

        let key = storage.add(obj, func_persistent);

        Ok(())
    }


    fn __eq__(&self, other: &Self) -> bool {
        self.key == other.key
    }


    fn __hash__(&self) -> u64 {
        let mut s = std::collections::hash_map::DefaultHasher::new();
        self.key.hash(&mut s);
        s.finish()
    }


    fn __repr__(&self) -> String {
        let pos = self.pos();
        let rot = self.rot();
        let scale  = self.scale();
        let has_tick_function = if self.function_key == None {false} else {true};
        format!("Cube(position={:?}, rotation={:?}, scale={:?}, has_tick_function={has_tick_function})", pos, rot,scale)
    }

    fn __str__(&self)-> String{
        let pos = self.pos();
        format!("Cube at ({:.2}, {:.2}, {:.2})", pos.x, pos.y, pos.z)
    }

}

impl Drop for Sphere{
    fn drop(&mut self) {

        // function storage MUST be cleaned first, since a function inside fun-storage may rely on the object still living.
        match self.function_key{
            None => {},
            Some(key)=> {
                let mut storage = ObjectFunctionStorage::get_fun_storage();
                storage.remove(key);
            }
        }

        COMMAND_QUEUE.push( Command::DeleteObject { key: self.key });
    }
}