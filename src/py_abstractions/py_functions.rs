// python abstractions for both macroquad functions and custom functions.
// All functions are either 
// 1) executed directly, if no engine context is needed
// 2) will be pushed into COMMAND_QUEUE to be executed by macroquad
//
// also, any conversion between my abstracted pyclasses and the structs used in macroquad is being done here.
// ( example:  Color -> mq::Color )

use super::py_structs::*;
use crate::py_abstractions::structs::Textures_and_Images::*;
use macroquad::prelude as mq;

use macroquad::text;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction; 
use pyo3_stub_gen::{derive::gen_stub_pyfunction};

use std::fmt::format;
use std::sync::mpsc;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::*;

use crate::COMMAND_QUEUE;
use crate::Command;
use crate::py_abstractions::structs::GLAM::Vec3::Vec3;
use crate::py_abstractions::structs::GLAM::Vec2::Vec2;
use crate::py_abstractions::Color::*;
use crate::py_abstractions::structs::KeyCode::*;

/// draws a rectangle with a given color.
/// viewing the rectangle required a 2D Camera ( default )
///
#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_rectangle(x: f32, y: f32, w: f32, h: f32, color: Color) {
    let c = mq::Color::new(color.r,color.g,color.b,color.a);
    COMMAND_QUEUE.push(Command::DrawRect { x, y, w, h,color:c});
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_affine_parallelepiped(offset: Vec3, e1: Vec3,e2: Vec3,e3: Vec3,texture: Option<Texture2D>,color: Color) {
    
    COMMAND_QUEUE.push(Command::DrawAfflineParallelpiped { offset: offset.into(), e1: e1.into(), e2: e2.into(), e3: e3.into(), texture: texture.map(Into::into), color: color.into() });
}


#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_arc( x: f32,
    y: f32,
    sides: u8,
    radius: f32,
    rotation: f32,
    thickness: f32,
    arc: f32,
    color: Color) {
    
    COMMAND_QUEUE.push(Command::DrawArc { x, y, sides, radius, rotation, thickness, arc, color: color.into() });
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_cube_wires( position: Vec3, size: Vec3, color: Color) {
    
    COMMAND_QUEUE.push(Command::DrawCubeWires {position: position.into(),size: size.into(),color: color.into()});
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_cylinder( position: Vec3,
    radius_top: f32,
    radius_bottom: f32,
    height: f32,
    texture: Option<Texture2D>,
    color: Color ) {
    
    COMMAND_QUEUE.push(Command::DrawCylinder {position: position.into(), radius_top: radius_top, radius_bottom: radius_bottom, 
        height, texture: texture.map(Into::into),color: color.into()});
}


/// loads a file from a given path.
/// works with web-assembly
#[gen_stub_pyfunction]
#[pyfunction]
pub fn load_file(path: String) -> PyResult<Vec<u8>> {
    let (sender, receiver) = mpsc::sync_channel(1);
    COMMAND_QUEUE.push(Command::LoadFile { path, sender });

    match receiver.recv() {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(e)) => Err(e.into()),
        Err(e) => panic!("Fatal MSPC Error:  {e}"),
    }
}



#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_cylinder_wires( position: Vec3,
    radius_top: f32,
    radius_bottom: f32,
    height: f32,
    texture: Option<Texture2D>,
    color: Color) {
    
    COMMAND_QUEUE.push(Command::DrawCylinderWires {position: position.into(), radius_top: radius_top.into(), radius_bottom: radius_bottom.into(), 
        height, texture: texture.map(Into::into),color: color.into()});
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_ellipse( x: f32, y: f32, w: f32, h: f32, rotation: f32, color: Color){
    COMMAND_QUEUE.push(Command::DrawEllipse { x, y, w, h, rotation, color: color.into()});
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_ellipse_lines( x: f32,
    y: f32,
    w: f32,
    h: f32,
    rotation: f32,
    thickness: f32,
    color: Color){
    COMMAND_QUEUE.push(Command::DrawEllipseLines { x, y, w, h, rotation, thickness, color: color.into()});
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_hexagon( x: f32,
    y: f32,
    size: f32,
    border: f32,
    vertical: bool,
    border_color: Color,
    fill_color: Color){
    COMMAND_QUEUE.push(Command::DrawHexagon { x, y, size, border, vertical, border_color: border_color.into(), fill_color: fill_color.into()});
}

#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_line_3d( start: Vec3, end: Vec3, color: Color){
    COMMAND_QUEUE.push(Command::DrawLine3D { start: start.into(), end: end.into(), color: color.into()});
}



/// draws a basic grid in 3d space.
/// requires a 3d camera to be seen.
///
#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_grid(slices: u32, spacing: f32, axes_color: Color, other_color: Color) {
    let c1 = mq::Color::new(axes_color.r,axes_color.g,axes_color.b,axes_color.a);
    let c2 = mq::Color::new(other_color.r,other_color.g,other_color.b,other_color.a);
    let c =Command::DrawGrid { slices, spacing, axes_color: c1, other_color: c2 };

    COMMAND_QUEUE.push(c );
}

/// draws a flat plane in 3d space.
/// requires a 3d camera to be seen.
///
#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_plane(center: Vec3, size: Vec2, color: Color, texture: Option<Texture2D>)  {
    let col = mq::Color::new(color.r,color.g,color.b,color.a);
    let cen = mq::vec3( center.x,center.y,center.z);
    let siz = mq::vec2(size.x,size.y);
    let tex = match texture {
        Some(t) => Some(  t.into()  ),
        None => None,
    };

    let c =Command::DrawPlane { center:cen,size:siz,color: col,texture: tex};

    COMMAND_QUEUE.push(c );
}

/// draws a basic 3d cube.
/// requires a 3d camera to be seen.
///
#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_cube(position: Vec3, size: Vec3, color: Color) {
    let col = mq::Color{r: color.r,g: color.g,b :color.b,a: color.a};
    let pos = mq::vec3(position.x,position.y,position.z);
    let siz = mq::vec3(size.x,size.y,size.z);
    let texture = None; // for now
    let c = mq::Color::new(color.r,color.g,color.b,color.a);
    COMMAND_QUEUE.push(  Command::DrawCube{pos: pos, size: siz, texture, color: c} );
}

/// fills the entire screen with a single color.
/// this is usually used at the start of a frame.
///
#[gen_stub_pyfunction]
#[pyfunction]
pub fn clear_background(color: Color) {
    let col = mq::Color{r: color.r,g: color.g,b: color.b,a: color.a};
    COMMAND_QUEUE.push(Command::ClearBackground { color: col});
}

/// processes all drawing commands that have accumulated.
/// blocks until the frame has been drawn.
///
/// also, this function cleans up dropped memory like textures
#[gen_stub_pyfunction]
#[pyfunction]
pub fn next_frame() {
    let (sender, receiver) = mpsc::sync_channel(1); // Create a blocking channel
    COMMAND_QUEUE.push(Command::NextFrame(sender));

    // Block until next frame is processed
    let _ = receiver.recv();
}

/// draws a text in 2d space.
/// requires a 2d camera to be seen.
///
#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_text(text: String, x: f32, y: f32, font_size: f32, color: Color) {
    let c = mq::Color::new(color.r,color.g,color.b,color.a);
    COMMAND_QUEUE.push(Command::DrawText {text, x, y, font_size, color:c});
}

/// draws very basic circle in 2d space.
/// requires a 2d camera to be seen.
///
/// note that this function simply draws a 20-sided polygon.
/// for a more "round" circle, simply call `draw_poly()` with a greater ammount of sides.
///
#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_circle(x: f32, y: f32, r: f32, color: Color) {
    let c = mq::Color::new(color.r,color.g,color.b,color.a);
    COMMAND_QUEUE.push(Command::DrawPoly{ x, y, sides:20, radius:r, rotation:0.0, color:c});
}

/// draws n-sided polygon in 2d space. 
/// increasing the polygon count will simply make it a circle.
/// requires a 2d camera to be seen.
///
#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_poly(x: f32, y: f32, sides: u8, radius: f32, rotation: f32, color: Color) {
    let c = mq::Color::new(color.r,color.g,color.b,color.a);
    COMMAND_QUEUE.push(Command::DrawPoly{ x, y, sides, radius, rotation, color: c});
}

/// draws a texture in 2d space.
/// requires a 2d-camera to be seen.
/// 
/// a texture gets created by calling `Texture2D.from_image( image )`
///
#[gen_stub_pyfunction]
#[pyfunction]
pub fn draw_texture(texture: Texture2D,x: f32, y: f32, color: Color ) {
    let c = mq::Color::new(color.r,color.g,color.b,color.a);
    let innerTexture: mq::Texture2D  = texture.into();
    COMMAND_QUEUE.push( Command::DrawTexture{ texture: innerTexture, x, y, color: c   }  );
   
}

/// returns the current frames per second
#[gen_stub_pyfunction]
#[pyfunction]
pub fn get_fps() -> i32 {
    let (sender, receiver) = mpsc::sync_channel(1);
    COMMAND_QUEUE.push(Command::GetFPS(sender));

    match receiver.recv() {
        Ok(fps) => fps,
        Err(e) => panic!("Fatal MSPC Error:  {e}"),
    }

}


/// returns an list of all keys that have been pressed since the last check.
/// pressed = key down + key up
#[gen_stub_pyfunction]
#[pyfunction]
pub fn get_keys_pressed() -> PyResult<KeyCodeSet> {
    //actually, i am not 100% sure how get_keys_pressed() works, so might wanna look into that.
    let (sender, receiver) = mpsc::sync_channel(1);
    COMMAND_QUEUE.push(Command::GetKeysPressed(sender));

    match receiver.recv() {
        Ok(keyset) => {
            let converted_keys: HashSet<KeyCode> = keyset
                .into_iter()
                .map(KeyCode::from)
                .collect();

            let k = KeyCodeSet::new(converted_keys);

            Ok(k)
        }
        Err(e) => panic!("Fatal MSPC Error:  {e}"),
    }
}


/// returns an list of all keys that have been released since the last check.
#[gen_stub_pyfunction]
#[pyfunction]
pub fn get_keys_released() -> KeyCodeSet {
    //actually, i am not 100% sure how get_keys_released() works, so might wanna look into that.
    let (sender, receiver) = mpsc::sync_channel(1);
    COMMAND_QUEUE.push(Command::GetKeysReleased(sender));

    match receiver.recv() {
        Ok(keyset) => {
            let converted_keys: HashSet<KeyCode> = keyset
                .into_iter()
                .map(KeyCode::from)
                .collect();

            let k = KeyCodeSet::new(converted_keys);

            k
        }
        Err(e) => panic!("Fatal MSPC Error:  {e}"),
    }
}


/// returns an list of all keys that are currently in the process of being pressed.
#[gen_stub_pyfunction]
#[pyfunction]
pub fn get_keys_down() -> KeyCodeSet {

    let (sender, receiver) = mpsc::sync_channel(1);
    COMMAND_QUEUE.push(Command::GetKeysDown(sender));

    match receiver.recv() {
        Ok(keyset) => {
            let converted_keys: HashSet<KeyCode> = keyset
                .into_iter()
                .map(KeyCode::from)
                .collect();

            let k = KeyCodeSet::new(converted_keys);

            k
        }
        Err(e) => panic!("Fatal MSPC Error:  {e}"),
    }

}



//none yet here

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