use macroquad::prelude::*;
use crate::engine::Objects::Cube::CubeMesh as qm;
pub const fn create_unit_cube() -> qm {
    const WHITE_: [u8; 4] = [255,255,255,255];
    let vertices = [
        // Front Face (Z+) - Normal: [0, 0, 1]
        Vertex { position: vec3(-0.5, -0.5,  0.5), uv: vec2(0.0, 0.0), color: WHITE_, normal: vec4(0.0, 0.0, 1.0, 0.0) },
        Vertex { position: vec3( 0.5, -0.5,  0.5), uv: vec2(1.0, 0.0), color: WHITE_, normal: vec4(0.0, 0.0, 1.0, 0.0) },
        Vertex { position: vec3( 0.5,  0.5,  0.5), uv: vec2(1.0, 1.0), color: WHITE_, normal: vec4(0.0, 0.0, 1.0, 0.0) },
        Vertex { position: vec3(-0.5,  0.5,  0.5), uv: vec2(0.0, 1.0), color: WHITE_, normal: vec4(0.0, 0.0, 1.0, 0.0) },

        // Back Face (Z-) - Normal: [0, 0, -1]
        Vertex { position: vec3( 0.5, -0.5, -0.5), uv: vec2(0.0, 0.0), color: WHITE_, normal: vec4(0.0, 0.0, -1.0, 0.0) },
        Vertex { position: vec3(-0.5, -0.5, -0.5), uv: vec2(1.0, 0.0), color: WHITE_, normal: vec4(0.0, 0.0, -1.0, 0.0) },
        Vertex { position: vec3(-0.5,  0.5, -0.5), uv: vec2(1.0, 1.0), color: WHITE_, normal: vec4(0.0, 0.0, -1.0, 0.0) },
        Vertex { position: vec3( 0.5,  0.5, -0.5), uv: vec2(0.0, 1.0), color: WHITE_, normal: vec4(0.0, 0.0, -1.0, 0.0) },

        // Top Face (Y+) - Normal: [0, 1, 0]
        Vertex { position: vec3(-0.5,  0.5,  0.5), uv: vec2(0.0, 0.0), color: WHITE_, normal: vec4(0.0, 1.0, 0.0, 0.0) },
        Vertex { position: vec3( 0.5,  0.5,  0.5), uv: vec2(1.0, 0.0), color: WHITE_, normal: vec4(0.0, 1.0, 0.0, 0.0) },
        Vertex { position: vec3( 0.5,  0.5, -0.5), uv: vec2(1.0, 1.0), color: WHITE_, normal: vec4(0.0, 1.0, 0.0, 0.0) },
        Vertex { position: vec3(-0.5,  0.5, -0.5), uv: vec2(0.0, 1.0), color: WHITE_, normal: vec4(0.0, 1.0, 0.0, 0.0) },

        // Bottom Face (Y-) - Normal: [0, -1, 0]
        Vertex { position: vec3(-0.5, -0.5, -0.5), uv: vec2(0.0, 0.0), color: WHITE_, normal: vec4(0.0, -1.0, 0.0, 0.0) },
        Vertex { position: vec3( 0.5, -0.5, -0.5), uv: vec2(1.0, 0.0), color: WHITE_, normal: vec4(0.0, -1.0, 0.0, 0.0) },
        Vertex { position: vec3( 0.5, -0.5,  0.5), uv: vec2(1.0, 1.0), color: WHITE_, normal: vec4(0.0, -1.0, 0.0, 0.0) },
        Vertex { position: vec3(-0.5, -0.5,  0.5), uv: vec2(0.0, 1.0), color: WHITE_, normal: vec4(0.0, -1.0, 0.0, 0.0) },

        // Right Face (X+) - Normal: [1, 0, 0]
        Vertex { position: vec3( 0.5, -0.5,  0.5), uv: vec2(0.0, 0.0), color: WHITE_, normal: vec4(1.0, 0.0, 0.0, 0.0) },
        Vertex { position: vec3( 0.5, -0.5, -0.5), uv: vec2(1.0, 0.0), color: WHITE_, normal: vec4(1.0, 0.0, 0.0, 0.0) },
        Vertex { position: vec3( 0.5,  0.5, -0.5), uv: vec2(1.0, 1.0), color: WHITE_, normal: vec4(1.0, 0.0, 0.0, 0.0) },
        Vertex { position: vec3( 0.5,  0.5,  0.5), uv: vec2(0.0, 1.0), color: WHITE_, normal: vec4(1.0, 0.0, 0.0, 0.0) },

        // Left Face (X-) - Normal: [-1, 0, 0]
        Vertex { position: vec3(-0.5, -0.5, -0.5), uv: vec2(0.0, 0.0), color: WHITE_, normal: vec4(-1.0, 0.0, 0.0, 0.0) },
        Vertex { position: vec3(-0.5, -0.5,  0.5), uv: vec2(1.0, 0.0), color: WHITE_, normal: vec4(-1.0, 0.0, 0.0, 0.0) },
        Vertex { position: vec3(-0.5,  0.5,  0.5), uv: vec2(1.0, 1.0), color: WHITE_, normal: vec4(-1.0, 0.0, 0.0, 0.0) },
        Vertex { position: vec3(-0.5,  0.5, -0.5), uv: vec2(0.0, 1.0), color: WHITE_, normal: vec4(-1.0, 0.0, 0.0, 0.0) },
    ];

    let indices = [
        0, 1, 2,  0, 2, 3,       // Front
        4, 5, 6,  4, 6, 7,       // Back
        8, 9, 10, 8, 10, 11,     // Top
        12, 13, 14, 12, 14, 15,  // Bottom
        16, 17, 18, 16, 18, 19,  // Right
        20, 21, 22, 20, 22, 23,  // Left
    ];

    qm {
        vertices,
        indices,
        texture: None,
    }
}