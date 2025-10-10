use macroquad::prelude as mq;
use macroquad::prelude::Mesh as mq_mesh;
use macroquad::prelude::Vertex as mq_vert;
use crate::py_abstractions::structs::Textures_and_Images::Texture2D;
use std::path::Path;

#[repr(C)]
pub struct Vertex {
    pos: [f32; 3],   // 12 bytes
    normal: [f32; 3],// 12 bytes
    uv: [f32; 2],    // 8 bytes
}

#[repr(C)]
pub struct Mesh{
	pub vertices: Vec<mq_vert>,
    pub indices: Vec<u16>,
    pub texture: Option<Texture2D>,
}

// impl From<Mesh> for mq_mesh {
//     fn from(v: Mesh) -> Self {
//         let t = v.texture.map(|texture| texture.texture);
//         mq_mesh {
//             vertices: v.vertices,
//             indices: v.indices,
//             texture: t,
//         }
//     }
// }


// impl From<mq_mesh> for Mesh {
//     fn from(m: mq_mesh) -> Self {
//         let t = m.texture.map(|texture| Texture2D { texture });
//         Mesh {
//             vertices: m.vertices,
//             indices: m.indices,
//             texture: t,
//         }
//     }
// }





//pub fn load_obj_mesh(path: &str) -> Result<Mesh, Box<dyn std::error::Error>> {
//    let (models, _) = tobj::load_obj(
//        Path::new(path),
//        &tobj::LoadOptions {
//            triangulate: true,
//            single_index: true,
//            ..Default::default()
//        },
//    )?;

//    let mut meshes = Vec::new();

//    for m in models {
//        let mesh = m.mesh;

//        let vertices: Vec<Vertex> = mesh.positions
//            .chunks_exact(3)
//            .enumerate()
//            .map(|(i, pos)| Vertex {
//                pos: [pos[0], pos[1], pos[2]],
//                normal: if !mesh.normals.is_empty() {
//                    let n = &mesh.normals[i * 3..i * 3 + 3];
//                    [n[0], n[1], n[2]]
//                } else {
//                    [0.0, 0.0, 0.0]
//                },
//                uv: if !mesh.texcoords.is_empty() {
//                    let uv = &mesh.texcoords[i * 2..i * 2 + 2];
//                    [uv[0], uv[1]]
//                } else {
//                    [0.0, 0.0]
//                },
//            })
//            .collect();

//        let indices: Vec<u16> = mesh.indices.iter().map(|&i| i as u16).collect();

//        meshes.push(Mesh {
//            vertices,
//            indices,
//            texture: None,
//        });
//    }

//    // Return the first mesh or an error if there are none
//    meshes.into_iter().next().ok_or_else(|| "No meshes found".into())
//}
