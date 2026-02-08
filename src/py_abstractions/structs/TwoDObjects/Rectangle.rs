use pyo3::prelude::*;
use pyo3_stub_gen::derive::* ;
use pyo3::types::{PyWeakref, PyWeakrefReference};
use pyo3::exceptions::*;

use crate::engine::PChannel::PChannel;
use crate::py_abstractions::structs::ThreeDObjects::PhysicsHandle::Physics;

use std::hash::{Hash, Hasher};

use slotmap::Key;
use crate::py_abstractions::structs::ThreeDObjects::ColliderOptions::InnerColliderOptions;
use crate::engine::Objects::ObjectDataCache;
use crate::engine::CoreLoop::COMMAND_QUEUE;
use crate::engine::CoreLoop::Command;

use crate::py_abstractions::structs::GLAM::Vec3::Vec3;
use crate::py_abstractions::structs::GLAM::Vec2::Vec2;
use crate::py_abstractions::structs::ThreeDObjects::ColliderOptions::ColliderOptions;
use crate::py_abstractions::structs::ThreeDObjects::ObjectFunctionStorage;
use crate::py_abstractions::Color::Color;
use crate::py_abstractions::structs::ThreeDObjects::ObjectFunctionStorage::FunctionKey;
use crate::engine::Objects::ObjectManagement::ObjectStorage::ObjectKey;


#[gen_stub_pyclass]
#[pyclass]
pub struct Rectangle{
    pub pos: Vec2,
    pub rot: Vec2,
    pub scale: Vec2,
}


#[gen_stub_pymethods]
#[pymethods]
impl Rectangle{
    #[new]
    pub fn new(pos: Vec2, rot: Vec2, scale: Vec2)-> Self{
        Rectangle { pos,rot,scale}
    }

    pub fn test2(&self){
        println!("Test2");
    }
}