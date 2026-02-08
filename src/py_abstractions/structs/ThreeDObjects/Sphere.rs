use pyo3::prelude::*;
use pyo3_stub_gen::derive::* ;
use pyo3::types::{PyWeakref, PyWeakrefReference};
use pyo3::exceptions::*;
use crate::engine::PChannel::PChannel;
use std::hash::{Hash, Hasher};

use slotmap::DefaultKey;
use slotmap::Key;

use crate::engine::Objects::ObjectDataCache;
use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;

use crate::py_abstractions::structs::GLAM::Vec3::Vec3;
use crate::py_abstractions::structs::ThreeDObjects::ColliderOptions::ColliderOptions;
use crate::py_abstractions::structs::ThreeDObjects::ObjectFunctionStorage;
use crate::py_abstractions::Color::Color;
use crate::py_abstractions::structs::ThreeDObjects::ObjectFunctionStorage::FunctionKey;
use crate::engine::Objects::ObjectManagement::ObjectStorage::ObjectKey;

#[gen_stub_pyclass]
#[pyclass(subclass, weakref)]
pub struct Sphere{
    key: ObjectKey, // The key to the actual underlying cube, stored inside "ObjectStorage".

    // Key to a function inside 'function storage', which will be run each frame by the engine.
    function_key: Option<FunctionKey>,

    /// we add a cache for trivial data, which can be used if the object is not
    /// influenced by outside forces F.E. Gravity.
    cache: Option<ObjectDataCache::ThreeDObjCache>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Sphere {

    #[pyo3(signature = (position= Vec3::ZERO(), rotation = Vec3::ZERO(),scale= Vec3::ONE(), color = Color::WHITE(), collider_type = ColliderOptions::NONE()))]
    #[new]
    pub fn new(
        py: Python<'_>,
        position: Vec3,
        rotation: Vec3,
        scale: Vec3,
        color: Color,
        collider_type: ColliderOptions,
    ) -> PyResult<Py<Sphere>> {

        let (sender, receiver) = PChannel::sync_channel(1);

        let cache = ObjectDataCache::ThreeDObjCache::new(
            true, position.into(), rotation.into(), scale.into(), color.into());
            
        let placeholder_struct: Sphere = Sphere { key: ObjectKey::null(),function_key: None,  cache};
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
            collider: collider_type,
            weak_ref: weak_ref_handle,
            sender 
        });
        
        let key = receiver.recv()?;
        
        cube_handle.borrow_mut(py).key = key;
        Ok(cube_handle) 
    }
        
    /// Accesses the scale of the given object.
    /// Note that individual values of an object can NOT be changed via:
    /// ```
    /// >>>object.scale.x += 1
    /// ```
    /// since object.scale returns a copy of its scale, one has to write:
    /// ```
    /// >>>object.scale += Vec3(1, 0, 0)
    /// ```
    #[getter]
    fn scale(&self) -> PyResult<Vec3> {
        if let Some(cache) = self.cache {
            return Ok(cache.scale.into())
        }

        let (sender, receiver) = PChannel::sync_channel(1);
        let command = Command::GetObjectScale { key: self.key, sender: sender };
        COMMAND_QUEUE.push(command);
        Ok(receiver.recv()?.into())
    }

    #[setter]
    fn set_scale(&mut self, value: Vec3) {
        if let Some(cache) = &mut self.cache {
            cache.scale = value.into();
        }

        let command = Command::SetObjectScale { key: self.key, scale: value.into() };
        COMMAND_QUEUE.push(command);
    }

    /// Accesses the position of the given object.
    /// Note that individual values of an object can NOT be changed via:
    /// ```
    /// >>>object.pos.x += 1
    /// ```
    /// since object.pos returns a copy of its position, one has to write:
    /// ```
    /// >>>object.pos += Vec3(1, 0, 0)
    /// ```
    #[getter]
    fn pos(&self) -> PyResult<Vec3> {
        if let Some(cache) = self.cache {
            return Ok(cache.position.into())
        }

        let (sender, receiver) = PChannel::sync_channel(1);
        let command = Command::GetObjectPos { key: self.key, sender: sender };
        COMMAND_QUEUE.push(command);
        Ok(receiver.recv()?.into())
    }

    #[setter]
    fn set_pos(&mut self, value: Vec3) {
        if let Some(cache) = &mut self.cache {
            cache.position = value.into();
        }
        let command = Command::SetObjectPos { key: self.key, position: value.into() };
        COMMAND_QUEUE.push(command);
    }

    /// Accesses the rotation of the given object.
    /// Note that individual values of an object can NOT be changed via:
    /// ```
    /// >>>object.rot.x += 1
    /// ```
    /// since object.rot returns a copy of its rotation, one has to write:
    /// ```
    /// >>>object.rot += Vec3(1, 0, 0)
    /// ```
    #[getter]
    fn rot(&self) -> PyResult<Vec3> {
        if let Some(cache) = self.cache {
            return Ok(cache.rotation.into())
        }

        let (sender, receiver) = PChannel::sync_channel(1);
        let command = Command::GetObjectRotation{ key: self.key, sender: sender };
        COMMAND_QUEUE.push(command);
        Ok(receiver.recv()?.into())
    }

    #[setter]
    fn set_rot(&mut self, value: Vec3) {
        if let Some(cache) = &mut self.cache {
            cache.rotation = value.into();
        }

        let command = Command::SetObjectRotation { key: self.key, rotation: value.into() };
        COMMAND_QUEUE.push(command);
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
    pub fn check_collision<'py>(&self, py: Python<'py> )-> PyResult<Vec<Bound<'py, PyAny>>>{

        let (sender, receiver) = PChannel::sync_channel(1);
        let command = Command::GetColissionObjects { key: self.key, sender };
        COMMAND_QUEUE.push(command);
        let res: Vec<std::sync::Arc<Py<PyWeakref>>> = receiver.recv()?;

        // we filter map, since any weakref we hold may already be invalid.
        Ok(res.into_iter().filter_map(|pyObj|{

            let weak_py: &Py<PyWeakref> = &*pyObj;

            let weak_bound: Bound<'py, PyWeakref> = weak_py.bind(py).clone();

            let upgraded: Bound<'py, PyAny> = weak_bound.call0().ok()?;

            if upgraded.is_none() {
                None
            } else {
                Some(upgraded)
            }
        }).collect())
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

    fn __str__(&self)-> PyResult<String>{
        let pos = self.pos()?;
        Ok(format!("Cube at ({:.2}, {:.2}, {:.2})", pos.x, pos.y, pos.z))
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