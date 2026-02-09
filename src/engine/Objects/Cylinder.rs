use macroquad::{color::Color, prelude::{self as mq,}};
use glam::{Vec3A, Mat3A, Quat, EulerRot};

#[derive( Debug, Clone)]
pub struct Cylinder{
    
    pub scale: mq::Vec3,
    pub position: mq::Vec3,
    pub rotation: mq::Vec3,
    pub color: mq::Color,

    pub mesh: CylinderMesh,

}
impl Cylinder{
    pub fn new(size: mq::Vec3, position: mq::Vec3, rotation: mq::Vec3, color: mq::Color)-> Cylinder{
        let mesh: CylinderMesh = CylinderMesh::new(size, position, rotation, None, color);

        Cylinder { scale: size,position,rotation,color,  mesh  }
    }

    pub fn draw(&self, gl: &mut macroquad::prelude::QuadGl ){
        gl.texture(None);
        gl.geometry(&self.mesh.vertices, &self.mesh.indices);
    
    }
}

#[derive(Clone, Debug)]
pub struct CylinderMesh{
    pub vertices: [macroquad::prelude::Vertex; 24],
    pub indices: [u16; 36],
    pub texture: Option<mq::Texture2D>,
}

impl CylinderMesh{
    pub fn new(
        size: mq::Vec3, 
        position: mq::Vec3, 
        rotation: mq::Vec3, 
        texture: Option<mq::Texture2D>, 
        color: mq::Color) -> Self {
            todo!()
        }
    pub fn recalculate_pos(&mut self, old_pos: mq::Vec3, new_pos: mq::Vec3){
        todo!()
    }
    pub fn recalculate_rot(&mut self, pivot: mq::Vec3, old_rot: mq::Vec3, new_rot: mq::Vec3){
        todo!()
    }
    pub fn recalculate_scale(&mut self, pivot: mq::Vec3, old_scale: mq::Vec3, new_scale: mq::Vec3){
        todo!()
    }
}