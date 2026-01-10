/// Pythonic quad GL.
/// 
/// 
/// 

/// Very unsafe very evil. use at your own expense!!!
///
/// 
use macroquad::prelude::*;
use miniquad::{Bindings,Pipeline,RenderPass,TextureId,UniformType};
use std::collections::BTreeMap;

// pub struct PQuadGl{
//     pub pipelines: PipelinesStorage,
//     pub state: GlState,
//     pub draw_calls: Vec<DrawCall>,
//     pub draw_calls_bindings: Vec<Bindings>,
//     pub draw_calls_count: usize,
//     pub start_time: f64,
//     pub white_texture: TextureId,
//     pub batch_vertex_buffer: Vec<Vertex>,
//     pub batch_index_buffer: Vec<u16>,
//     pub max_vertices: usize,
//     pub max_indices: usize,
// }
// impl PQuadGl{
//     pub unsafe fn from(gl: &mut mq::QuadGl)-> &mut PQuadGl{
//         let p  = 
//     }
//     pub unsafe fn to(&mut self)-> &mut mq::QuadGl{

//     }
// }