#![allow(warnings)]

use crossbeam::queue::SegQueue;
use std::process;
use lazy_static::*;

use pyo3::prelude::*;
use pyo3_stub_gen::{derive::gen_stub_pyfunction, define_stub_info_gatherer,derive::*};

use std::sync::Mutex;
use macroquad::prelude as mq;

mod engine;
use engine::load_ressources as eng;

mod py_abstractions;
use py_abstractions::py_structs::*;
use py_abstractions::py_functions::*;
use std::io::Error;
use std::sync::mpsc;
use std::collections::HashSet;
use std::sync::Arc;
use crate::py_abstractions::structs::Textures_and_Images as structs;
use crate::py_abstractions::structs::Camera as Camera;
use crate::py_abstractions::Mouse as Mouse;
use crate::engine::SHADERS::shaderLoader;
use crate::engine::SHADERS::shader_manager as sm;
use crate::py_abstractions::Color::*;

lazy_static! {
    pub static ref COMMAND_QUEUE: Arc<SegQueue<Command>> = Arc::new(SegQueue::new());
    
}


pub enum Command {

    SetDefaultCamera(),

    DrawRect { x: f32, y: f32, w: f32, h: f32, color: mq::Color },

    DrawPlane { center: mq::Vec3, size: mq::Vec2, color: mq::Color, texture: Option<mq::Texture2D> },

    DrawGrid { slices: u32, spacing: f32, axes_color: mq::Color, other_color: mq::Color },

    DrawCube { pos: mq::Vec3, size: mq::Vec3, texture: Option<mq::Texture2D>, color: mq::Color},

    DrawCubemap { pos: mq::Vec3, size: mq::Vec3, texture: Option<mq::Texture2D>, color: mq::Color},

    DrawPoly{ x: f32, y: f32, sides: u8, radius: f32, rotation: f32, color: mq::Color},

    DrawText{text: String, x: f32, y: f32, font_size: f32, color: mq::Color},

    DrawTexture{ texture: mq::Texture2D, x: f32, y: f32, color: mq::Color   },
    
    ClearBackground { color: mq::Color },

    GetFPS(mpsc::SyncSender<i32>),

    GetMousePosition(mpsc::SyncSender<(f32,f32)>),
    
    Get_Keys_Pressed(mpsc::SyncSender<HashSet<mq::KeyCode>>),

    Get_Keys_Down(mpsc::SyncSender<HashSet<mq::KeyCode>>),

    Get_Keys_Released(mpsc::SyncSender<HashSet<mq::KeyCode>>),

    NextFrame(mpsc::SyncSender<()>),

    ImgToTexture {
        image: Arc<mq::Image>,
        sender: mpsc::SyncSender<mq::Texture2D>,
    },

    LoadImage {
        path: String,
        sender: mpsc::SyncSender<Result<mq::Image, macroquad::Error>>,
    },

    SetCamera{camera_2d: Option<mq::Camera2D>, camera_3d: Option<mq::Camera3D>},

    SetCursorGrab ( bool ),

    ShowMouse(bool),
    
}


lazy_static! {
    pub static ref ComputationTime: Arc<Vec<u128>> = Arc::new(Vec::new());

}

async fn process_commands() {
    // processes commands that rely on the macroquad engine
    // commands that do not rely on it's core (openGL) components are found elsewhere.

    while let Some(command) = COMMAND_QUEUE.pop() {
        match command {
            Command::DrawRect { x, y, w, h, color} => {
                sm::switch_to_desired_shader(sm::ShaderKind::None);
                mq::draw_rectangle(
                    x,
                    y,
                    w,
                    h,
                    color,
                );
            }
            Command::DrawPlane { center, size, color, texture } => {
                sm::switch_to_desired_shader(sm::ShaderKind::None);
                let tex_ref = texture.as_ref();
                mq::draw_plane(center,size,tex_ref,color);
            }
            Command::DrawGrid { slices, spacing, axes_color, other_color } => {
                sm::switch_to_desired_shader(sm::ShaderKind::None);
                mq::draw_grid(slices, spacing, axes_color, other_color);
            }
            Command::DrawCube {pos,size,texture,color} => {
                sm::switch_to_desired_shader(sm::ShaderKind::Basic);
                mq::draw_cube(pos,size,texture.as_ref(),color)
            }
            Command::DrawCubemap {pos,size,texture,color} => {
                sm::switch_to_desired_shader(sm::ShaderKind::None);
                mq::draw_cube(pos,size,texture.as_ref(),color)
            }
            
            Command::LoadImage { path,sender} => {
                let result = async {
                let bytes = mq::load_file(&path).await?;
                
                let image = mq::Image::from_file_with_format(&bytes, None)?;
                Ok(image)
            }.await;

                let _ = sender.send(result);
            }
            Command::DrawPoly { x, y, sides, radius, rotation, color }=>
            {
                sm::switch_to_desired_shader(sm::ShaderKind::None);
                mq::draw_poly(x,y,sides,radius,rotation,color  );
            }
            Command::SetDefaultCamera() =>{ mq::set_default_camera() }

            Command::DrawTexture { texture,x,y,color}=>
            {
                sm::switch_to_desired_shader(sm::ShaderKind::None);
                mq::draw_texture(&texture,x,y,color)
            }
            
            Command::ClearBackground {color } => {
                macroquad::prelude::clear_background(color);
            }

            Command::NextFrame(sender) => {
                mq::next_frame().await;
                engine::SHADERS::shader_manager::new_frame_shader_update();
                let _ = sender.send(());
            }
         
            Command::DrawText { text, x, y, font_size, color }=>{
               sm::switch_to_desired_shader(sm::ShaderKind::None);
               mq::draw_text(text.as_str(), x,y,font_size,color);
            }
            
            Command::GetFPS(sender) => {
            let fps = mq::get_fps();
            let _ = sender.send(fps);
            }
            Command::GetMousePosition(sender) => {
            let pos = mq::mouse_position();
            let _ = sender.send(pos);
            }
            Command::Get_Keys_Pressed(sender) => {
            let keyset = mq::get_keys_pressed();
            let _ = sender.send(keyset);
            }
            Command::Get_Keys_Released(sender) => {
            let keyset = mq::get_keys_released();
            let _ = sender.send(keyset);
            }
            Command::Get_Keys_Down(sender) => {
            let keyset = mq::get_keys_down();
            let _ = sender.send(keyset);
            }
            Command::ImgToTexture { image, sender }=>{
                let tex: mq::Texture2D = mq::Texture2D::from_image(&image);
                let _ = sender.send(tex);
            }
            Command::SetCamera { camera_2d, camera_3d } => { // merged cam2d and 3d for simplicity.
                match (camera_2d, camera_3d) {
                    (Some(cam), None) => mq::set_camera(&cam),
                    (None, Some(cam)) => mq::set_camera(&cam),
                    _ => panic!("invalid cam pattern"),

                }
            }
            Command::SetCursorGrab(i) =>{
                mq::set_cursor_grab(i);
            }
            Command::ShowMouse(i) =>{
                mq::show_mouse(i);
            }
            
        }
    }

   
}
/// [!] This should generally be the first function call.
///
/// Turns on the pyquad engine, creates an open-gl window and allows for engine-calls to be processed.
///
#[gen_stub_pyfunction]
#[pyfunction]
#[pyo3(signature = (conf = None))] // overloads activate_engine with config
fn activate_engine(_py: Python, conf: Option<Config>) {
    match conf {
        Some(config) => {
            let mut miniConf = mq::miniquad::conf::Conf {
                    window_title: config.window_title,
                    window_width: config.window_width,
                    window_height: config.window_height,
                    fullscreen: config.fullscreen,

                    ..Default::default()
            };

            /// Optional swap interval (vertical sync).
            ///
            /// Note that this is highly platform- and driver-dependent.
            /// There is no guarantee the FPS will match the specified `swap_interval`.
            /// In other words, `swap_interval` is only a hint to the GPU driver and
            /// not a reliable way to limit the game's FPS.
            if !config.vsync {  miniConf.platform.swap_interval = Some(0);}

            let mut macroConf = macroquad::conf::Conf {
                miniquad_conf: miniConf,
                update_on: Some(macroquad::conf::UpdateTrigger::default()),
                default_filter_mode: macroquad::prelude::FilterMode::Linear,
                draw_call_vertex_capacity: 10000,
                draw_call_index_capacity: 5000,
            };


            std::thread::spawn(move || {
                macroquad::Window::from_config(macroConf, async {
                    engine::EngineSetup::setup_engine();
                    loop { 
                        process_commands().await;
                    }
                });
                if config.stop_pyton_when_closing_window{
                    println!("Pyquad window closed. Exiting process.");
                    process::exit(0);
                }
            });
        }
        None => {
            std::thread::spawn(|| {
                macroquad::Window::new("pyquad", async {
                    engine::EngineSetup::setup_engine();
                    loop {
                        process_commands().await;
                    }
                });
                println!("Pyquad window closed. Exiting process.");
                process::exit(0);
            });
            
        }
    }
}

#[pymodule]
fn pyquad( py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> { // exposes all functionality to python

    // functions
    m.add_function(wrap_pyfunction!(activate_engine, m)?)?;
    m.add_function(wrap_pyfunction!(draw_rectangle, m)?)?;
    m.add_function(wrap_pyfunction!(draw_poly, m)?)?;
    m.add_function(wrap_pyfunction!(draw_circle, m)?)?;
    m.add_function(wrap_pyfunction!(next_frame, m)?)?;
    m.add_function(wrap_pyfunction!(clear_background, m)?)?;
    m.add_function(wrap_pyfunction!(draw_text, m)?)?;
    m.add_function(wrap_pyfunction!(get_fps, m)?)?;
    m.add_function(wrap_pyfunction!(get_keys_pressed, m)?)?;
    m.add_function(wrap_pyfunction!(get_keys_down, m)?)?;
    m.add_function(wrap_pyfunction!(get_keys_released, m)?)?;
    m.add_function(wrap_pyfunction!(draw_grid, m)?)?;
    m.add_function(wrap_pyfunction!(draw_texture, m)?)?;
    m.add_function(wrap_pyfunction!(draw_plane, m)?)?;
    m.add_function(wrap_pyfunction!(engine::Cubemap::draw_cubemap, m)?)?;

    m.add_function(wrap_pyfunction!(draw_cube, m)?)?;
    m.add_function(wrap_pyfunction!(Mouse::get_mouse_position, m)?)?;
    m.add_function(wrap_pyfunction!(Mouse::set_cursor_grab, m)?)?;
    m.add_function(wrap_pyfunction!(Mouse::show_mouse, m)?)?;

    m.add_function(wrap_pyfunction!(Camera::set_default_camera, m)?)?;
    
    m.add_class::<structs::Texture2D>()?;
    m.add_class::<structs::Image>()?;
    m.add_class::<Camera::Camera2D>()?;
    m.add_class::<Camera::Camera3D>()?;

    m.add_class::<Config>()?;
    m.add_class::<DVec2>()?;
    m.add_class::<DVec3>()?;
    m.add_class::<DVec4>()?;
    m.add_class::<Circle>()?;
    m.add_class::<Quat>()?;
    m.add_class::<Rect>()?;

    m.add_class::<Color>()?;
    m.add_class::<DMat2>()?;
    m.add_class::<DMat3>()?;
    m.add_class::<DMat4>()?;

    m.add_class::<py_abstractions::structs::GLAM::BVec3::BVec3>()?;
    m.add_class::<py_abstractions::structs::GLAM::Vec3::Vec3>()?;
    m.add_class::<py_abstractions::structs::GLAM::Vec2::Vec2>()?;




    //extra
    m.add_class::<crate::py_abstractions::structs::KeyCode::KeyCode>()?;
    m.add_class::<crate::py_abstractions::structs::KeyCode::KeyCodeSet>()?;


    Ok(())

    
    
}

define_stub_info_gatherer!(stub_info);

/*

list of macroquad enums

   mq::Comparison
    mq::DrawMode
    mq::EulerRot
    mq::FilterMode
    mq::ImageFormat
    mq::KeyCode
    mq::MouseButton
    mq::Projection
    mq::ShaderError
    mq::ShaderSource
    mq::TouchPhase
    mq::UniformType




*/





/*
list of macroquad::prelude functions



    mq::build_textures_atlas
    mq::camera_font_scale
    mq::cartesian_to_polar
    mq::clamp
    mq::clear_background
    mq::clear_input_queue

    mq::draw_affine_parallelepiped
    mq::draw_affine_parallelogram
    mq::draw_arc
    mq::draw_circle
    mq::draw_circle_lines
    mq::draw_cube
    mq::draw_cube_wires
    mq::draw_cylinder
    mq::draw_cylinder_ex
    mq::draw_cylinder_wires
    mq::draw_ellipse
    mq::draw_ellipse_lines
    mq::draw_fps
    mq::draw_grid
    mq::draw_grid_ex
    mq::draw_hexagon
    mq::draw_line
    mq::draw_line_3d
    mq::draw_mesh
    mq::draw_multiline_text
    mq::draw_plane
    mq::draw_poly
    mq::draw_poly_lines
    mq::draw_rectangle
    mq::draw_rectangle_ex
    mq::draw_rectangle_lines
    mq::draw_rectangle_lines_ex
    mq::draw_sphere
    mq::draw_sphere_ex
    mq::draw_sphere_wires

    mq::draw_text
    mq::draw_text_ex

    mq::draw_texture
    mq::draw_texture_ex
    mq::draw_triangle
    mq::draw_triangle_lines
    mq::get_char_pressed
    mq::get_dropped_files
    mq::get_fps
    mq::get_frame_time
    mq::get_internal_gl
    mq::get_keys_down
    mq::get_keys_pressed
    mq::get_keys_released
    mq::get_last_key_pressed
    mq::get_screen_data
    mq::get_text_center
    mq::get_time
    mq::gl_use_default_material
    mq::gl_use_material
    mq::is_key_down
    mq::is_key_pressed
    mq::is_key_released
    mq::is_mouse_button_down
    mq::is_mouse_button_pressed
    mq::is_mouse_button_released
    mq::is_quit_requested
    mq::is_simulating_mouse_with_touch
    mq::load_file
    mq::load_image
    mq::load_material
    mq::load_string
    mq::load_texture
    mq::load_ttf_font_from_bytes
    mq::measure_text
    mq::mouse_delta_position
    mq::mouse_position
    mq::mouse_position_local
    mq::mouse_wheel
    mq::next_frame
    mq::polar_to_cartesian
    mq::pop_camera_state
    mq::prevent_quit
    mq::push_camera_state
    mq::quat
    mq::render_target
    mq::render_target_ex
    mq::render_target_msaa
    mq::request_new_screen_size
    mq::screen_dpi_scale
    mq::screen_height
    mq::screen_width
    mq::set_camera
    mq::set_cursor_grab
    mq::set_default_camera
    mq::set_default_filter_mode
    mq::set_fullscreen
    mq::set_panic_handler
    mq::set_pc_assets_folder
    mq::show_mouse
    mq::simulate_mouse_with_touch
    mq::touches
    mq::touches_local


*/