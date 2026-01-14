use macroquad::{color::Color, prelude as mq, window::InternalGlContext};
use rapier3d::prelude::*;

#[derive( Debug, Clone)]
pub struct Cube{
    
    pub scale: mq::Vec3,
    pub position: mq::Vec3,
    pub rotation: mq::Vec3,
    pub color: mq::Color,

    pub mesh: CubeMesh,

}
impl Cube {
    pub fn new(size: mq::Vec3, position: mq::Vec3, rotation: mq::Vec3, color: mq::Color)-> Cube{
        let mesh: CubeMesh = CubeMesh::new(size, position, rotation, None, color);

        Cube { scale: size,position,rotation,color,  mesh  }
    }
    pub fn draw(&self, gl: &mut macroquad::prelude::QuadGl ){
            gl.texture(None);
            gl.geometry(&self.mesh.vertices, &self.mesh.indices);
        
    }


}



#[derive(Clone, Debug)]
pub struct CubeMesh{
    pub vertices: [macroquad::prelude::Vertex; 24],
    pub indices: [u16; 36],
    pub texture: Option<mq::Texture2D>,
}

impl CubeMesh {
    pub fn new(
        size: mq::Vec3, 
        position: mq::Vec3, 
        rotation: mq::Vec3, 
        texture: Option<mq::Texture2D>, 
        color: mq::Color) -> Self {

        use mq::{Mat4, Vec2, Vec3, Vertex};

        let hs = size * 0.5;

        let positions = [
            Vec3::new(-hs.x, -hs.y, -hs.z),
            Vec3::new(hs.x, -hs.y, -hs.z),
            Vec3::new(hs.x, hs.y, -hs.z),
            Vec3::new(-hs.x, hs.y, -hs.z),
            Vec3::new(-hs.x, -hs.y, hs.z),
            Vec3::new(hs.x, -hs.y, hs.z),
            Vec3::new(hs.x, hs.y, hs.z),
            Vec3::new(-hs.x, hs.y, hs.z),
        ];

        let rot = Mat4::from_euler(mq::EulerRot::XYZ, rotation.x, rotation.y, rotation.z);
        let transform = Mat4::from_translation(position) * rot;

        // Each face: 4 indices (quad)
        let faces = [
            ([0, 1, 2, 3], Vec3::new(0.0, 0.0, -1.0)),
            ([5, 4, 7, 6], Vec3::new(0.0, 0.0, 1.0)),
            ([4, 0, 3, 7], Vec3::new(-1.0, 0.0, 0.0)),
            ([1, 5, 6, 2], Vec3::new(1.0, 0.0, 0.0)),
            ([3, 2, 6, 7], Vec3::new(0.0, 1.0, 0.0)),
            ([4, 5, 1, 0], Vec3::new(0.0, -1.0, 0.0)),
        ];

        let uvs = [
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
        ];

        let mut vertices= [Vertex {
            position: mq::Vec3::ZERO,
            uv: mq::Vec2::ZERO,
            color: color.into(),
            normal: mq::Vec4::ZERO,
        }; 24];

        let mut vi = 0;
        for (face, normal) in faces.iter() {
            for (j, &corner) in face.iter().enumerate() {
                let pos = (transform * positions[corner].extend(1.0)).truncate();
                vertices[vi] = Vertex {
                    position: mq::vec3(pos.x, pos.y, pos.z),
                    uv: mq::vec2(uvs[j].x, uvs[j].y),
                    color: color.into(),
                    normal:  mq::vec4(normal.x, normal.y, normal.z, 0.0),
                };
                vi += 1;
            }
        }

        let mut indices = [0u16; 36];
        for i in 0..6 {
            let base = (i * 4) as u16;
            let offset = i * 6;
            indices[offset + 0] = base;
            indices[offset + 1] = base + 1;
            indices[offset + 2] = base + 2;
            indices[offset + 3] = base;
            indices[offset + 4] = base + 2;
            indices[offset + 5] = base + 3;
        }

        Self {
            vertices,
            indices,
            texture,
        }
    }

}