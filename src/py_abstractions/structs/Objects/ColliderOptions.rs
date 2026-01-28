use pyo3::prelude::*;
use crate::py_abstractions::structs::GLAM::Vec3::Vec3;
use pyo3_stub_gen::derive::* ;


/// A Settings-Class for Colliders.
/// the following options exist:
/// 
/// ```
/// ...# no collision or physics
/// >>>ColliderOptions.NONE 
/// ...
/// ...# collision but no physics
/// >>>ColliderOptions.STATIC
/// ...
/// ...# both physics and collision.
/// >>>ColliderOptions.DYNAMIC(...)
/// ```
#[gen_stub_pyclass]
#[pyclass(frozen)]
#[derive(Clone, Copy)]
pub struct ColliderOptions(pub InnerColliderOptions);

#[gen_stub_pymethods]
#[pymethods]
impl ColliderOptions{


    #[classattr]
    pub fn NONE() -> ColliderOptions {
        ColliderOptions(InnerColliderOptions::None)
    }

    #[classattr]
    pub fn STATIC() -> ColliderOptions {
        ColliderOptions(InnerColliderOptions::Static)
    }
    
    #[staticmethod]
    pub fn DYNAMIC(gravity: Vec3) -> ColliderOptions {
        ColliderOptions(InnerColliderOptions::Dynamic { gravity })
    }
}

#[derive(Clone,Copy)]
pub enum InnerColliderOptions{
    None,
    Static,
    Dynamic{
        gravity: Vec3,
    }
}