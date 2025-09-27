
use glam::Vec2 as gl;

use pyo3::prelude::*;
use pyo3_stub_gen::{derive::gen_stub_pyfunction, define_stub_info_gatherer,derive::*};


#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone, Copy, PartialEq,Debug)]
pub struct Vec2 {
    #[pyo3(get, set)]
    pub x: f32,
    #[pyo3(get, set)]
    pub y: f32,

}

impl Vec2 {
    // Const constructor for compile-time constants
    #[inline(always)]
    pub const fn const_new(x: f32, y: f32) -> Self {
    Vec2 { x, y }
    }


    // Const constructor for splat values
    #[inline(always)]
    pub const fn splat(value: f32) -> Self {
    Vec2 { x: value, y: value }
    }
}


#[gen_stub_pymethods]
#[pymethods]
impl Vec2 {
    #[new]
    pub fn new(x: f32, y: f32) -> Self {
    Self { x, y }
    }


    pub fn normalize(&self) -> Self {
    let a: gl = (*self).into();
    a.normalize().into()
    }
}






impl From<Vec2> for gl {
    fn from(v: Vec2) -> Self {
        gl::new(v.x, v.y)
    }
}


impl From<gl> for Vec2 {
    fn from(v: gl) -> Self {
        Vec2 {
        x: v.x,
        y: v.y,
        }
    }
}