use macroquad::prelude::Vertex as mq_vert;
use pyo3::{pyclass, pymethods};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use crate::py_abstractions::structs::Textures_and_Images::Texture2D;

#[gen_stub_pyclass]
#[pyclass]
#[repr(C)]
pub struct Vertex {
    pos: [f32; 3],
    normal: [f32; 3],
    uv: [f32; 2],
}

#[gen_stub_pyclass]
#[pyclass]
#[repr(C)]
pub struct Mesh{
	pub vertices: Vec<mq_vert>,
    pub indices: Vec<u16>,
    pub texture: Option<Texture2D>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Mesh{
    #[new]
    fn new()-> Self{
        todo!()
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Vertex{
    #[new]
    fn new()-> Self{
        todo!()
    }
}
