use macroquad::prelude::*;
// Ensure we use the Miniquad types correctly
use macroquad::miniquad::{self, BufferLayout, BufferSource, BufferType, BufferUsage, Pipeline, PipelineParams, ShaderId, VertexAttribute, VertexFormat, VertexStep};
use glam::{Mat4, Quat, Vec3};

use crate::engine::Objects::{Cube::Cube, ObjectManagement::ObjectStorage::*};

// --- The Renderer Struct ---

pub struct CubeInstancer {
    pipeline: Pipeline,
    bindings: miniquad::Bindings,
    capacity: usize,
    instance_data: Vec<Mat4>, 
}

impl CubeInstancer {
    pub fn new() -> Self {
        let ctx = unsafe { get_internal_gl() }.quad_context;

        // 1. Geometry (Standard 1x1x1 Cube)
        #[rustfmt::skip]
        let vertices: [f32; 24 * 3] = [
            -0.5, -0.5,  0.5,   0.5, -0.5,  0.5,   0.5,  0.5,  0.5,  -0.5,  0.5,  0.5,
            -0.5, -0.5, -0.5,   0.5, -0.5, -0.5,   0.5,  0.5, -0.5,  -0.5,  0.5, -0.5,
            -0.5,  0.5,  0.5,   0.5,  0.5,  0.5,   0.5,  0.5, -0.5,  -0.5,  0.5, -0.5,
            -0.5, -0.5,  0.5,   0.5, -0.5,  0.5,   0.5, -0.5, -0.5,  -0.5, -0.5, -0.5,
             0.5, -0.5,  0.5,   0.5, -0.5, -0.5,   0.5,  0.5, -0.5,   0.5,  0.5,  0.5,
            -0.5, -0.5,  0.5,  -0.5, -0.5, -0.5,  -0.5,  0.5, -0.5,  -0.5,  0.5,  0.5,
        ];

        #[rustfmt::skip]
        let indices: [u16; 36] = [
             0,  1,  2,  2,  3,  0,
             5,  4,  7,  7,  6,  5,
             8,  9, 10, 10, 11,  8,
            13, 12, 15, 15, 14, 13,
            16, 17, 18, 18, 19, 16,
            21, 20, 23, 23, 22, 21,
        ];

        let vertex_buffer = ctx.new_buffer(BufferType::VertexBuffer, BufferUsage::Immutable, BufferSource::slice(&vertices));
        let index_buffer = ctx.new_buffer(BufferType::IndexBuffer, BufferUsage::Immutable, BufferSource::slice(&indices));

        let capacity = 10_000;
        let instance_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Stream,
            BufferSource::slice(&vec![Mat4::IDENTITY; capacity]),
        );

        let shader_id = ctx.new_shader(
            miniquad::ShaderSource::Glsl { vertex: VERTEX_SHADER, fragment: FRAGMENT_SHADER },
            miniquad::ShaderMeta {
                images: vec![], 
                uniforms: miniquad::UniformBlockLayout { uniforms: vec![miniquad::UniformDesc::new("vp", miniquad::UniformType::Mat4)] },
            }
        ).expect("Shader compilation failed");

        let pipeline = ctx.new_pipeline(
            &[
                BufferLayout { stride: 12, ..Default::default() },
                BufferLayout { stride: 64, step_func: VertexStep::PerInstance, step_rate: 1 },
            ],
            &[
                VertexAttribute::new("pos", VertexFormat::Float3),
                VertexAttribute::new("m0", VertexFormat::Float4),
                VertexAttribute::new("m1", VertexFormat::Float4),
                VertexAttribute::new("m2", VertexFormat::Float4),
                VertexAttribute::new("m3", VertexFormat::Float4),
            ],
            shader_id,
            PipelineParams {
                depth_test: miniquad::PipelineParams::default().depth_test,
                ..Default::default()
            },
        );

        let bindings = miniquad::Bindings {
            vertex_buffers: vec![vertex_buffer, instance_buffer],
            index_buffer,
            images: vec![],
        };

        Self { pipeline, bindings, capacity, instance_data: Vec::with_capacity(capacity) }
    }

    pub fn push(&mut self, position: Vec3, size: Vec3, rotation: Vec3) {
        let rotation_quat = Quat::from_euler(glam::EulerRot::XYZ, rotation.x, rotation.y, rotation.z);
        let mat = Mat4::from_scale_rotation_translation(size, rotation_quat, position);
        self.instance_data.push(mat);
    }

    // CHANGED: Now accepts the ViewProjection matrix explicitly
    pub fn draw(&mut self, view_proj: Mat4) {
        if self.instance_data.is_empty() { return; }

        let ctx = unsafe { get_internal_gl() }.quad_context;
        let count = self.instance_data.len();

        if count > self.capacity {
            self.capacity = count * 2;
            ctx.delete_buffer(self.bindings.vertex_buffers[1]);
            self.bindings.vertex_buffers[1] = ctx.new_buffer(
                BufferType::VertexBuffer,
                BufferUsage::Stream,
                BufferSource::slice(&vec![Mat4::IDENTITY; self.capacity]),
            );
        }

        ctx.buffer_update(self.bindings.vertex_buffers[1], BufferSource::slice(&self.instance_data));

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        
        // Pass the explicitly provided matrix to the shader
        ctx.apply_uniforms(miniquad::UniformsSource::table(&ShaderUniforms { vp: view_proj }));
        
        ctx.draw(0, 36, count as i32);
        self.instance_data.clear();
    }
}

const VERTEX_SHADER: &str = r#"#version 330 core
    in vec3 pos;
    in vec4 m0;
    in vec4 m1;
    in vec4 m2;
    in vec4 m3;
    uniform mat4 vp;
    out vec3 v_color;
    void main() {
        mat4 model = mat4(m0, m1, m2, m3);
        gl_Position = vp * model * vec4(pos, 1.0);
        v_color = vec3(0.5) + pos; 
    }
"#;

const FRAGMENT_SHADER: &str = r#"#version 330 core
    in vec3 v_color;
    out vec4 FragColor;
    void main() {
        FragColor = vec4(v_color, 1.0);
    }
"#;

#[repr(C)]
struct ShaderUniforms { vp: Mat4 }