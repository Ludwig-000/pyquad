use pyo3::prelude::*;
use pyo3_stub_gen::derive::* ;
use pyo3::types::{PyWeakref, PyWeakrefReference};
use pyo3::exceptions::*;

use crate::{implement_Drop, implement_basic_3D_getter_methods, implement_basic_3D_magic_methods, implement_basic_3D_setter_methods, implement_check_collision, implement_remove_tick, implement_set_collider, implement_tick};
use crate::engine::PChannel::PChannel;
use crate::py_abstractions::structs::ThreeDObjects::PhysicsHandle::Physics;
use std::hash::{Hash, Hasher};

use slotmap::Key;
use crate::py_abstractions::structs::ThreeDObjects::ColliderOptions::InnerColliderOptions;
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
pub struct Cube{
    key: ObjectKey, // The key to the actual underlying cube, stored inside "ObjectStorage".

    // Key to a function inside 'function storage', which will be run each frame by the engine.
    function_key: Option<FunctionKey>,

    /// we add a cache for trivial data, which can be used if the object is not
    /// influenced by outside forces F.E. Gravity.
    cache: Option<ObjectDataCache::ThreeDObjCache>,

    /// a collection of physics-related methods, that can be applied to the object.
    /// 'physics' will be set to Some() if the object is innitialized as a dynamic object.
    #[pyo3(get)]
    pub physics: Option<Py<Physics>>,
}


crate::implement_basic_3D_magic_methods!(Cube);
crate::implement_basic_3D_getter_methods!(Cube);
crate::implement_basic_3D_setter_methods!(Cube);
crate::implement_check_collision!(Cube);
crate::implement_set_collider!(Cube);
crate::implement_tick!(Cube,  r#"Cube()"#);
crate::implement_remove_tick!(Cube);
crate::implement_Drop!(Cube);

#[gen_stub_pymethods]
#[pymethods]
impl Cube {

    #[pyo3(signature = (position= Vec3::ZERO(), rotation = Vec3::ZERO(),scale= Vec3::ONE(), color = Color::WHITE(), collider_type = ColliderOptions::NONE()))]
    #[new]
    pub fn new(
        py: Python<'_>,
        position: Vec3,
        rotation: Vec3,
        scale: Vec3,
        color: Color,
        collider_type: ColliderOptions,
    ) -> PyResult<Py<Cube>> {

        let (sender, receiver) = PChannel::sync_channel(1);

        let cache  =match collider_type.0{
            InnerColliderOptions::Dynamic { gravity_scale, friction, restitution, density }=>{
                None
            },
            _=> {ObjectDataCache::ThreeDObjCache::new(true, position.into(), rotation.into(), scale.into(), color.into())}
        };
        let placeholder_struct: Cube = Cube { key: ObjectKey::null(),function_key: None,  cache, physics: None};
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
            collider: collider_type,
            weak_ref: weak_ref_handle.clone_ref(py),
            sender 
        });
        
        let key = receiver.recv()?;
        
        let mut cube_ref = cube_handle.borrow_mut(py);
        cube_ref.key = key;


        match collider_type.0{
            InnerColliderOptions::Dynamic { gravity_scale, friction, restitution, density }=>{
                let phys_struct = Physics {
                    identity: weak_ref_handle,
                    handle: key,
                };
                cube_ref.physics = Some(Py::new(py, phys_struct)?);
            },
            _ => {}
        }

        drop(cube_ref);

        Ok(cube_handle)
    }
}
