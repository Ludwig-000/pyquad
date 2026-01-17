use macroquad::{color::Color, prelude as mq, window::InternalGlContext};
use glam::{Vec3A, Mat3A, Quat, EulerRot};
// This fix for Error #1: Explicitly import the enum
use gltf::mesh::util::ReadIndices;

pub struct Mesh{
    
    pub scale: mq::Vec3,
    pub position: mq::Vec3,
    pub rotation: mq::Vec3,
    pub color: mq::Color,

    pub mesh: mq::Mesh,

}
impl Mesh{

    pub fn load_from_gltf(path: &str, texture: Option<mq::Texture2D>) -> Option<Self> {
        let (document, buffers, _) = gltf::import(path).ok()?;
        
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for mesh in document.meshes() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                // 1. Extract Positions
                let positions: Vec<_> = reader.read_positions()?.map(mq::Vec3::from).collect();
                
                // 2. Extract Normals (Fix for Error #3)
                let normals: Vec<_> = reader
                    .read_normals()
                    .map(|n| n.map(mq::Vec3::from).collect())
                    .unwrap_or_else(|| vec![mq::vec3(0.0, 1.0, 0.0); positions.len()]);

                // 3. Extract UVs
                let tex_coords: Vec<_> = reader
                    .read_tex_coords(0)
                    .map(|uv| uv.into_f32().map(|v| mq::vec2(v[0], v[1])).collect())
                    .unwrap_or_else(|| vec![mq::vec2(0.0, 0.0); positions.len()]);

                // 4. Extract Colors
                let colors: Vec<_> = reader
                    .read_colors(0)
                    .map(|c| c.into_rgba_f32().map(|rgba| mq::Color::from_vec(mq::vec4(rgba[0], rgba[1], rgba[2], rgba[3]))).collect())
                    .unwrap_or_else(|| vec![mq::WHITE; positions.len()]);

                // Map to Macroquad Vertices (including the normal field)
                for i in 0..positions.len() {
                    vertices.push(mq::Vertex {
                        position: positions[i],
                        uv: tex_coords[i],
                        color: colors[i].into(),
                        normal: normals[i].extend(0.0), // Added normal field here
                    });
                }

                // Extract Indices
                if let Some(read_indices) = reader.read_indices() {
                    match read_indices {
                        ReadIndices::U16(iter) => indices.extend(iter),
                        ReadIndices::U32(iter) => indices.extend(iter.map(|i| i as u16)),
                        ReadIndices::U8(iter) => indices.extend(iter.map(|i| i as u16)),
                    }
                }
            }
        }

        Some(Self {
            scale: mq::vec3(1.0, 1.0, 1.0),
            position: mq::vec3(0.0, 0.0, 0.0),
            rotation: mq::vec3(0.0, 0.0, 0.0),
            color: mq::WHITE,
            mesh: mq::Mesh {
                vertices,
                indices,
                texture,
            },
        })
    }
}