#![allow(warnings)]

use crossbeam::queue::SegQueue;
use macroquad::texture::RenderTarget;
use std::fmt::format;
use std::process;
use std::panic;

use pyo3::prelude::*;
use pyo3_stub_gen::{derive::gen_stub_pyfunction, define_stub_info_gatherer,derive::*};

use lazy_static::*;

use std::sync::Mutex;
use macroquad::prelude as mq;
use macroquad::audio as au;

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
use crate::py_abstractions::structs::Config::*;
use pyo3::exceptions::PyUnicodeWarning;
use crate::engine::PError::PError;

lazy_static! {
    pub static ref COMMAND_QUEUE: Arc<SegQueue<Command>> = Arc::new(SegQueue::new());
    
}


pub enum Command {

    LoadFile{ path: String, sender: mpsc::SyncSender<Result<Vec<u8>, PError>> },
    LoadSound{ path: String, sender: mpsc::SyncSender<Result<au::Sound, PError>> },
    
    LoadSoundFromBytes{data: Vec<u8>, sender: mpsc::SyncSender<Result<au::Sound, PError>> },

    PlaySound{ sound: au::Sound, params: au::PlaySoundParams },

    PlaySoundOnce{ sound: au::Sound },

    SetSoundVolume{ sound: au::Sound, volume: f32 },

    StopSound{ sound: au::Sound },

    RenderTargetMsaa{ width: u32, height: u32, sender: mpsc::SyncSender<mq::RenderTarget> },
    RenderTargetEx{ width: u32, height: u32, params: Option<mq::RenderTargetParams>, sender: mpsc::SyncSender<mq::RenderTarget> },
    DrawArc{ x: f32,
    y: f32,
    sides: u8,
    radius: f32,
    rotation: f32,
    thickness: f32,
    arc: f32,
    color: mq::Color,},
    DrawCubeWires{position: mq::Vec3, size: mq::Vec3, color: mq::Color},
    DrawCylinder{ position: mq::Vec3,
    radius_top: f32,
    radius_bottom: f32,
    height: f32,
    texture: Option<mq::Texture2D>,
    color: mq::Color,},
    DrawCylinderWires{ position: mq::Vec3,
        radius_top: f32,
        radius_bottom: f32,
        height: f32,
        texture: Option<mq::Texture2D>,
        color: mq::Color,},

    DrawEllipse{x: f32, y: f32, w: f32, h: f32, rotation: f32, color: mq::Color},
    DrawEllipseLines{x: f32,
            y: f32,
            w: f32,
            h: f32,
            rotation: f32,
            thickness: f32,
            color: mq::Color,},
    DrawHexagon{x: f32,
                y: f32,
                size: f32,
                border: f32,
                vertical: bool,
                border_color: mq::Color,
                fill_color: mq::Color,},
    DrawLine3D{start: mq::Vec3, end: mq::Vec3, color: mq::Color},


    DrawAfflineParallelpiped{offset: mq::Vec3, e1: mq::Vec3,e2: mq::Vec3,e3: mq::Vec3,texture: Option<mq::Texture2D>,color: mq::Color},
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
    
    GetKeysPressed(mpsc::SyncSender<HashSet<mq::KeyCode>>),

    GetKeysDown(mpsc::SyncSender<HashSet<mq::KeyCode>>),

    GetKeysReleased(mpsc::SyncSender<HashSet<mq::KeyCode>>),

    NextFrame(mpsc::SyncSender<()>),

    ImgToTexture {
        image: Arc<mq::Image>,
        sender: mpsc::SyncSender<mq::Texture2D>,
    },

    LoadImage {
        path: String,
        sender: mpsc::SyncSender<Result<mq::Image, PError>>,
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
                mq::draw_cube(pos,size,texture.as_ref(),color);
                crate::engine::Cubemap::draw_fullscreen_quad();

            }

            Command::DrawAfflineParallelpiped { offset, e1, e2, e3, texture, color } => {
                mq::draw_affine_parallelepiped(offset, e1, e2, e3, texture.as_ref(), color);
            }
            Command::DrawArc { x, y, sides, radius, rotation, thickness, arc, color } => {
                mq::draw_arc(x, y, sides, radius, rotation, thickness, arc, color);
            }

            Command::DrawCubeWires { position, size, color } => {
                mq::draw_cube_wires(position, size, color);
            }
            Command::DrawCylinder { position, radius_top, radius_bottom, height, texture, color } => {
                mq::draw_cylinder(position, radius_top, radius_bottom, height, texture.as_ref(), color);
            }

            Command::DrawCylinderWires { position, radius_top, radius_bottom, height, texture, color } => {
                mq::draw_cylinder_wires(position, radius_top, radius_bottom, height, texture.as_ref(), color);
            }

            Command::DrawEllipse { x, y, w, h, rotation, color }    => {
                mq::draw_ellipse(x, y, w, h, rotation, color);  
            }
            
            Command::DrawEllipseLines { x, y, w, h, rotation, thickness, color }    => {
                mq::draw_ellipse_lines(x, y, w, h, rotation, thickness, color);
            }

            Command::DrawHexagon { x, y, size, border, vertical, border_color, fill_color } => {
                mq::draw_hexagon(x, y, size, border, vertical, border_color, fill_color);
            }

            Command::DrawLine3D { start, end, color } =>{
                mq::draw_line_3d(start, end, color);    
            }
            
            // end of todo


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
            Command::GetKeysPressed(sender) => {
            let keyset = mq::get_keys_pressed();
            let _ = sender.send(keyset);
            }
            Command::GetKeysReleased(sender) => {
            let keyset = mq::get_keys_released();
            let _ = sender.send(keyset);
            }
            Command::GetKeysDown(sender) => {
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
            Command::RenderTargetMsaa{ width, height, sender } => {
                let render_target = mq::render_target_msaa(width, height);
                let _ = sender.send(render_target);
            }
            Command::RenderTargetEx{ width, height, params, sender } => {
                match (params){
                    Some(p) => {
                        let render_target = mq::render_target_ex(width, height, p);
                        let _ = sender.send(render_target);
                    }
                    None => {
                        let render_target = mq::render_target_ex(width, height, mq::RenderTargetParams::default());
                       let _ = sender.send(render_target);
                    }
                }

            }
            Command::LoadSound { path ,sender} => {
                let result = async {
                    let data = macroquad::prelude::load_file(&path).await?;

                    //converts .mp3 to .wav
                    let secured_data  = engine::AudioConverter::ensure_wav(data).map_err(|e| {
                       e.with_context(format!(" path: {path}"))
                    }  )?;

                    let sound = au::load_sound_from_bytes(&secured_data).await?;
                    Ok(sound)
                }.await;
            
                let _ = sender.send(result);
            }
            Command::LoadSoundFromBytes { data ,sender} => {

                 let result: Result<_, PError> = async {
                    //converts .mp3 to .wav
                    let secured_data  = engine::AudioConverter::ensure_wav(data)?;

                    let sound = au::load_sound_from_bytes(&secured_data).await?;
                    Ok(sound)
                }.await;
            
                let _ = sender.send(result);
            }
            
            Command::PlaySound { sound, params } => {
                au::play_sound(&sound, params);
            }
            Command::PlaySoundOnce { sound } => {
                au::play_sound_once(&sound);
            }
            Command::StopSound { sound } => {
                au::stop_sound(&sound);
            }
            Command::SetSoundVolume { sound, volume  } => {
                au::set_sound_volume(&sound, volume);
            }

            Command::LoadFile { path ,sender} => {

                let result = mq::load_file(&path).await.map_err(Into::into);
                let _ = sender.send(result);

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
pub fn activate_engine(_py: Python, conf: Option<Config>) {
    match conf {
        Some(config) => {

            let mut macroConf = Config::to_window_config(config.clone());

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
    m.add_function(wrap_pyfunction!(activate_engine, m)?)?;
    // functions

    m.add_function(wrap_pyfunction!(draw_rectangle, m)?)?;
    m.add_function(wrap_pyfunction!(draw_poly, m)?)?;
    m.add_function(wrap_pyfunction!(draw_circle, m)?)?;
    m.add_function(wrap_pyfunction!(draw_affine_parallelepiped, m)?)?;
    m.add_function(wrap_pyfunction!(draw_arc, m)?)?;
    m.add_function(wrap_pyfunction!(draw_cube_wires, m)?)?;
    m.add_function(wrap_pyfunction!(draw_cylinder, m)?)?;
    m.add_function(wrap_pyfunction!(draw_cylinder_wires, m)?)?;
    m.add_function(wrap_pyfunction!(draw_ellipse, m)?)?;
    m.add_function(wrap_pyfunction!(draw_ellipse_lines, m)?)?;
    m.add_function(wrap_pyfunction!(draw_hexagon, m)?)?;
    m.add_function(wrap_pyfunction!(draw_line_3d, m)?)?;


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
    m.add_function(wrap_pyfunction!(py_abstractions::py_functions::load_file, m)?)?;

    m.add_function(wrap_pyfunction!(draw_cube, m)?)?;
    m.add_function(wrap_pyfunction!(Mouse::get_mouse_position, m)?)?;
    m.add_function(wrap_pyfunction!(Mouse::set_cursor_grab, m)?)?;
    m.add_function(wrap_pyfunction!(Mouse::show_mouse, m)?)?;

    m.add_function(wrap_pyfunction!(Camera::set_default_camera, m)?)?;
    
    m.add_class::<structs::Texture2D>()?;
    m.add_class::<structs::Image>()?;
    m.add_class::<Camera::Camera2D>()?;
    m.add_class::<Camera::Camera3D>()?;


    m.add_class::<crate::py_abstractions::structs::RenderTarget::RenderTarget>()?;
    m.add_class::<crate::py_abstractions::structs::RenderTarget::RenderTargetParams>()?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::structs::RenderTarget::render_target_msaa, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_abstractions::structs::RenderTarget::render_target, m)?)?;


    m.add_class::<crate::py_abstractions::structs::Audio::PlaySoundParams>()?;
    m.add_class::<crate::py_abstractions::structs::Audio::Sound>()?;

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