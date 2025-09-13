// main file used for debugging and testing. Not relevant to the project.


use macroquad::prelude::*;

use macroquad::material::{Material, MaterialParams};
use macroquad::window::miniquad::{
    ShaderSource, PipelineParams, Comparison, UniformDesc, UniformType, CullFace,
};

const VERTEX_SHADER: &str = r#"
#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
attribute vec4 normal;

uniform mat4 Model;
uniform mat4 Projection;

varying lowp vec4 v_color;
varying highp vec3 v_world_pos;
varying mediump vec3 v_normal;

void main() {
    vec4 world_pos = Model * vec4(position, 1.0);
    v_world_pos = world_pos.xyz;
    v_color = color0 / 255.0;

    if (length(normal.xyz) > 1e-6) {
        v_normal = normalize((Model * vec4(normal.xyz, 0.0)).xyz);
    } else {
        v_normal = vec3(0.0);
    }

    gl_Position = Projection * world_pos;
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 100
precision highp float;

#ifdef GL_OES_standard_derivatives
#extension GL_OES_standard_derivatives : enable
#endif

varying lowp vec4 v_color;
varying highp vec3 v_world_pos;
varying mediump vec3 v_normal;

uniform vec3 LightDir;

void main() {
    vec3 N = v_normal;

    if (length(N) < 1e-5) {
        #ifdef GL_OES_standard_derivatives
            vec3 dx = dFdx(v_world_pos);
            vec3 dy = dFdy(v_world_pos);
            N = normalize(cross(dx, dy));
        #else
            N = vec3(0.0, 1.0, 0.0); // Fallback if derivatives aren't available
        #endif
    }

    vec3 L = normalize(LightDir);
    float ndotl = dot(N, L) * 0.5 + 0.5;
    float brightness = 0.3 + 0.5 * ndotl;
    vec3 color = v_color.rgb * brightness;
    gl_FragColor = vec4(color, v_color.a);
}
"#;

const SKYBOX_VS: &str = r#"#version 100
attribute vec3 position;
varying vec3 v_dir;

uniform mat4 View;
uniform mat4 Projection;

void main() {
    // The vertex position of the cube is used as the direction vector for sampling the texture.
    v_dir = position;

    // Transform the cube's vertex by the camera's rotation (View) and projection.
    vec4 pos = Projection * View * vec4(position, 1.0);
    
    // This is the key trick: by setting z and w to the same value, the depth after
    // perspective division becomes 1.0, pushing the skybox to the far clipping plane.
    gl_Position = pos.xyww;
}
"#;

const SKYBOX_FS: &str = r#"#version 100
precision highp float;

varying vec3 v_dir;
uniform sampler2D u_tex;

const float PI = 3.14159265359;

// Map a 3D direction to equirectangular UVs
vec2 dir_to_uv(vec3 d) {
    d = normalize(d);
    float lon = atan(d.z, d.x);      // atan(y, x) is the robust atan2
    float lat = asin(d.y);           // -pi/2..pi/2
    return vec2(0.5 + lon / (2.0 * PI),
                0.5 - lat / PI);
}

void main() {
    vec2 uv = dir_to_uv(v_dir);
    gl_FragColor = texture2D(u_tex, uv);
}
"#;

pub fn load_custom_material() -> Material {
    let pipeline_params = PipelineParams {
        depth_write: true,
        depth_test: Comparison::LessOrEqual,
        ..Default::default()
    };

    let material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER,
        },
        MaterialParams {
            pipeline_params,
            uniforms: vec![UniformDesc::new("LightDir", UniformType::Float3)],
            ..Default::default()
        },
    )
    .unwrap();
    
    let elev = 15f32.to_radians();
    let azim = 45f32.to_radians(); // NE
    let lx = elev.cos() * azim.cos();
    let ly = elev.sin();
    let lz = elev.cos() * azim.sin();
    let light_dir = vec3(lx, ly, lz);

    // Set the uniform on the material
    material.set_uniform("LightDir", (light_dir.x, light_dir.y, light_dir.z));

    material
}

fn window_conf() -> Conf {
    Conf {
        window_title: "3D Camera and Skybox".to_owned(),
        window_width: 1920,
        window_height: 1080,
        fullscreen: true, // Start in windowed mode for easier debugging
        sample_count: 8,
        ..Default::default()
    }
}
fn draw_fullscreen_quad(){

}
#[macroquad::main(window_conf)]
async fn main() {
    // 1) Load any 2:1 panorama image; PNG/JPG is fine
    let sky_tex = load_texture("sky.png").await.unwrap();
    sky_tex.set_filter(FilterMode::Linear);

    let sky_mat = load_material(
        ShaderSource::Glsl {
            vertex: SKYBOX_VS,
            fragment: SKYBOX_FS,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("View", UniformType::Mat4),
                UniformDesc::new("Projection", UniformType::Mat4),
            ],
            textures: vec!["u_tex".into()],
            pipeline_params: PipelineParams {
                depth_write: false,
                depth_test: Comparison::LessOrEqual,
                cull_face: CullFace::Front, // Render inside of the cube
                ..Default::default()
            },
        },
    )
    .unwrap();
    
    // The `set_texture` function takes ownership of the Texture2D, so we don't pass a reference.
    sky_mat.set_texture("u_tex", sky_tex);


    // Load the material for the other objects
    let custom_material = load_custom_material();

    let mut cam_pos = vec3(0.0, 2.0, 5.0);
    let mut yaw = 0.0_f32;
    let mut pitch = 0.0_f32;

    let move_speed = 0.1;
    let mouse_sensitivity = 0.002;

    let mut last_mouse_pos = mouse_position();

    set_cursor_grab(true);
    show_mouse(false);

    loop {
        // --- Camera and Input Logic (Unchanged) ---
        let current_mouse_pos = mouse_position();
        let delta_x = current_mouse_pos.0 - last_mouse_pos.0;
        let delta_y = current_mouse_pos.1 - last_mouse_pos.1;
        last_mouse_pos = current_mouse_pos;

        yaw += delta_x * mouse_sensitivity;
        pitch -= delta_y * mouse_sensitivity;
        pitch = pitch.clamp(-1.54, 1.54);

        let forward = vec3(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalize();

        let right = vec3(forward.z, 0.0, -forward.x).normalize();

        if is_key_down(KeyCode::W) { cam_pos += forward * move_speed; }
        if is_key_down(KeyCode::S) { cam_pos -= forward * move_speed; }
        if is_key_down(KeyCode::D) { cam_pos -= right * move_speed; }
        if is_key_down(KeyCode::A) { cam_pos += right * move_speed; }
        if is_key_down(KeyCode::Space) { cam_pos.y += move_speed; }
        if is_key_down(KeyCode::LeftControl) { cam_pos.y -= move_speed; }

        let cam_target = cam_pos + forward;

        // --- Drawing ---
        clear_background(LIGHTGRAY);

        // --- COMPILER FIX 3 ---
        // Create the camera struct first.
        let cam = Camera3D {
            position: cam_pos,
            up: vec3(0.0, 1.0, 0.0),
            target: cam_target,
            fovy: 70.0,
            projection: Projection::Perspective,
            ..Default::default()
        };
        
        // Set this as the active camera for drawing objects.
        set_camera(&cam);

        // Manually calculate the projection and view matrices.
        // This is necessary because older macroquad versions don't have `cam.projection_matrix()`
        let aspect_ratio = screen_width() / screen_height();
        let projection_matrix = Mat4::perspective_rh(cam.fovy.to_radians(), aspect_ratio, cam.z_near, cam.z_far);
        let view_matrix = Mat4::look_at_rh(cam.position, cam.target, cam.up);
        
        // Remove the translation part from the view matrix for the skybox.
        let mut view_no_translation = view_matrix;
        view_no_translation.w_axis = vec4(0.0, 0.0, 0.0, 1.0);

        // Set the correct uniforms on the skybox material.
        sky_mat.set_uniform("View", view_no_translation);
        sky_mat.set_uniform("Projection", projection_matrix);

        // Draw the skybox
        gl_use_material(&sky_mat);
        draw_cube(vec3(0.0, 0.0, 0.0), vec3(100.0, 100.0, 100.0), None, WHITE);
        gl_use_default_material();
        gl_use_material(&custom_material);
        for i in 0..100{
            for o in 0..100{
                draw_cube(vec3(i as f32, 0.0, o as f32), vec3(0.8, 0.8, 0.8), None, YELLOW)
            }
        }
        gl_use_default_material();


        draw_grid(20, 1.0, GREEN, DARKGREEN);
        draw_fullscreen_quad();

        gl_use_material(&custom_material);
        draw_cube(vec3(0.0, 0.5, 0.0), vec3(1.0, 1.0, 1.0), None, RED);
        draw_sphere(vec3(5.0, 0.5, 0.0), 1.0, None, RED);
        gl_use_default_material();


        set_default_camera();
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 30.0, BLACK);

        next_frame().await;
    }
}
