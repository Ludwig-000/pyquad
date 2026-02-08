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

crate::implement_basic_3D_magic_methods!(Sphere);
crate::implement_basic_3D_getter_methods!(Sphere);
crate::implement_basic_3D_setter_methods!(Sphere);
crate::implement_check_collision!(Sphere);
crate::implement_set_collider!(Sphere);
crate::implement_tick!(Sphere);
crate::implement_remove_tick!(Sphere);
crate::implement_Drop!(Sphere);

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
        
        

}