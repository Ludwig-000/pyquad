
use crate::py_abstractions::structs::Textures_and_Images::*;
use macroquad::prelude as mq;

use pyo3::prelude::*;
 
use pyo3_stub_gen::{derive::gen_stub_pyfunction};


use crate::COMMAND_QUEUE;
use crate::Command;

#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_cubemap(texture: Texture2D) {
    let col = mq::Color::new(1.,1.,1.,1.);
    let pos = mq::vec3(0.,0.,0.);
    let siz = mq::vec3(10000.0, 10000.0,10000.0); // f32::INFINITY
    let texture_unpacked = texture.into();
    COMMAND_QUEUE.push(  Command::DrawCubemap{pos: pos, size: siz, texture: Some(texture_unpacked), color: col} );
}



pub fn draw_fullscreen_quad(){

    let vertices = vec![
        mq::Vertex::new(-1., -1., 0., 0., 0.,  mq::RED),
        mq::Vertex::new( 1., -1., 0., 1., 0.,  mq::RED),
        mq::Vertex::new( 1.,  1., 0., 1., 1.,  mq::RED),
        mq::Vertex::new(-1.,  1., 0., 0., 1.,  mq::RED),
        
    ];
    
    let indices = vec![0,1,2, 0,2,3];
    let mesh  =mq::Mesh { vertices, indices, texture: None };
    mq::draw_mesh(&mesh);
    //mq::render_target(width, height)
}