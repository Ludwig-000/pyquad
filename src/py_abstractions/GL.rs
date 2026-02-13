use std::cell::RefCell;

///
/// 
/// 
/// A bunch of functions and structs to mimic "get_internal_gl" and it's functionality
/// 
/// 
/// 



use macroquad::prelude as mq;
use pyo3::{pyclass, pymethods};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

use crate::py_abstractions::{Color::Color, structs::GLAM::Vec2::Vec2, structs::GLAM::Vec3::Vec3};

#[gen_stub_pyclass]
#[pyclass(frozen)]
#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct Vertex {
    pub position: mq::Vec3,
    pub uv: mq::Vec2,
    pub color: [u8; 4],
    /// Normal is not used by macroquad and is completely optional.
    /// Might be usefull for custom shaders.
    /// While normal is not used by macroquad, it is completely safe to use it
    /// to pass arbitary user data, hence Vec4.
    pub normal: mq::Vec4,
}

impl Vertex{
    pub fn new(position: Vec3, uv: Vec2, color: Color)-> Vertex{
        Vertex{ 
            position: position.into(),
             uv: uv.into(),
            color: [
                (color.r * 255.)as u8,
                (color.g * 255.)as u8,
                (color.b * 255.)as u8,
                (color.a * 255.)as u8,
            ],
            normal: mq::Vec4::ZERO,
        }
    }
}



impl Into<mq::Vertex> for Vertex{
    fn into(self) -> mq::Vertex {
        mq::Vertex::new2(self.position, self.uv, self.color.into())
    }
}


/// NOT YET IMPLEMENTED
#[gen_stub_pyclass(module = "pyroquad.internal_gl")] // module = .. does not seem to do anything but does not err?
#[pyclass]
pub struct InternalGL();

#[gen_stub_pymethods]
#[pymethods]
impl InternalGL{
    #[staticmethod]
    pub fn clear_draw_calls(){
        todo!()
    }
    #[staticmethod]
    pub fn delete_pipeline(){
        todo!()
    }
    #[staticmethod]
    pub fn depth_test(){
        todo!()
    }
    #[staticmethod]
    pub fn draw(){
        todo!()
    }
    #[staticmethod]
    pub fn draw_mode(){
        todo!()
    }
    #[staticmethod]
    pub fn geometry(){
        todo!()
    }
    #[staticmethod]
    pub fn get_active_render_pass(){
        todo!()
    }
    #[staticmethod]
    pub fn get_viewport_matrix(){
        todo!()
    }
    #[staticmethod]
    pub fn get_viewport(){
        todo!()
    }
    #[staticmethod]
    pub fn is_depth_test_enabled(){
        todo!()
    }
    #[staticmethod]
    pub fn make_pipeline(){
        todo!()
    }
    #[staticmethod]
    pub fn pipeline(){
        todo!()
    }
    #[staticmethod]
    pub fn push_model_matrix(){
        todo!()
    }
    #[staticmethod]
    pub fn pop_model_matrix(){
        todo!()
    }
    #[staticmethod]
    pub fn render_pass(){
        todo!()
    }
    #[staticmethod]
    pub fn reset(){
        todo!()
    }
    #[staticmethod]
    pub fn scissor(){
        todo!()
    }
    #[staticmethod]
    pub fn set_texture(){
        todo!()
    }
    #[staticmethod]
    pub fn set_uniform(){
        todo!()
    }
    #[staticmethod]
    pub fn set_uniform_array(){
        todo!()
    }
    #[staticmethod]
    pub fn texture(){
        todo!()
    }
    #[staticmethod]
    pub fn viewport(){
        todo!()
    }

}