use macroquad::prelude::Vertex as mq_vert;
use pyo3::ffi::PyCallable_Check;
use pyo3::{IntoPyObjectExt, pyclass, pymethods};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

use pyo3_stub_gen::inventory::submit;
use slotmap::DefaultKey;
use slotmap::Key;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use crate::engine::Objects::ObjectDataCache::ThreeDObjCache;
use crate::engine::PError::PError;
use crate::py_abstractions::structs::Objects::ColliderOptions::ColliderOptions;
use crate::py_abstractions::{Loading::FileData::FileData, structs::Textures_and_Images::Texture2D};
use crate::py_abstractions::structs::GLAM::Vec3::Vec3;
use crate::engine::Objects::Mesh as internal_mesh;
use pyo3::prelude::PyResult;
use crate::engine::CoreLoop::{Command,COMMAND_QUEUE};
use pyo3::prelude::*;

use pyo3::ffi;
use crate::engine::PChannel::PChannel;

use pyo3::prelude::*;
use pyo3_stub_gen::derive::* ;
use pyo3::types::{PyWeakref, PyWeakrefReference};
use crate::py_abstractions::structs::Objects::ObjectFunctionStorage::FunctionKey;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::* ;

use pyo3::exceptions::*;



use crate::engine::Objects::ObjectDataCache;

use crate::py_abstractions::structs::Objects::ObjectFunctionStorage;
use crate::py_abstractions::Color::Color;
use crate::engine::Objects::ObjectManagement::ObjectStorage::ObjectKey;
use pyo3::PyRef;

/// Physics that are bound to a Dynamic 3D object.
/// This class can only be used by making a Dynamic 3D object, and accessing it's 'physics' field.
#[gen_stub_pyclass]
#[cfg_attr(feature = "abi_314", pyclass(frozen, immutable_type))]
#[cfg_attr(not(feature = "abi_314"), pyclass(frozen))]
pub struct Physics{
    identity: Py<PyWeakref>,
    handle: ObjectKey
}

#[pymethods]
impl Physics{
    pub fn set_linear_velocity(&self, velocity: Vec3) {
         todo!() 
    }
    pub fn get_linear_velocity(&self) -> Vec3 {
         todo!() 
    }
    pub fn set_angular_velocity(&self, velocity: Vec3) {
         todo!() 
    }
    pub fn get_angular_velocity(&self) -> Vec3 {
         todo!() 
    }
    pub fn apply_impulse(&self, impulse: Vec3){
        todo!()
    }
    pub fn apply_force(&self, force: Vec3){
        todo!()
    }
    pub fn add_torque(&self, force: Vec3){
        todo!()
    }
    pub fn add_torque_impulse(&self, impulse: Vec3){
        todo!()
    }
    pub fn lock_rotation_x_Axes(&self, set: bool){
        todo!()
    }
    pub fn lock_rotation_y_Axes(&self, set: bool){
        todo!()
    }
    pub fn lock_rotation_z_Axes(&self, set: bool){
        todo!()
    }
    pub fn set_gravity_scale(&self, gravity: f32){
        todo!()
    }
    pub fn set_mass(&self, mass: f32){
        todo!()
    }
    pub fn set_friction(&self, friction: f32){
        todo!()
    }
    pub fn set_restitution(&self, restitiution: f32){
        todo!()
    }
    pub fn set_density(&self, density: f32){
        todo!()
    }
    pub fn set_linear_damping(&self, damping: f32){
        todo!()
    }
    pub fn set_angular_damping(&self, damping: f32){
        todo!()
    }
    /// continuous collision detection.
    pub fn enable_ccd(&self, enabled: bool) { todo!() }
}


impl Physics{
    pub fn is_alive<'py>(&self, py: Python<'py>)-> bool{
        unsafe {
            let weak_ptr = self.identity.as_ptr();
            let target_ptr = ffi::PyWeakref_GetObject(weak_ptr);
            !target_ptr.is_null() && target_ptr != ffi::Py_None()
        }
    }
}