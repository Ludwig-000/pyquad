
use crate::py_abstractions::structs::Textures_and_Images::*;
use macroquad::prelude as mq;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction; 
use pyo3_stub_gen::{derive::gen_stub_pyfunction};

use std::sync::mpsc;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::*;

use crate::COMMAND_QUEUE;
use crate::Command;

#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_cubemap(texture: Texture2D) {
    let col = mq::Color::new(1.,1.,1.,1.);
    let pos = mq::vec3(0.,0.,0.);
    let siz = mq::vec3(10000.0, 10000.0,10000.0); // f32::INFINITY
    let texture_unpacked = texture.texture;
    COMMAND_QUEUE.push(  Command::DrawCubemap{pos: pos, size: siz, texture: Some(texture_unpacked), color: col} );
}