use std::panic;
use std::sync::mpsc;
use std::collections::HashSet;
use std::sync::Arc;
use std::any::Any;
use pyo3::prelude::PyResult;
use crossbeam::channel::SendTimeoutError;
use lazy_static::*;

use crossbeam::queue::SegQueue;

use macroquad::prelude as mq;
use macroquad::audio as au;
use slotmap::DefaultKey;

use crate::engine::Objects::Mesh::Mesh;
use crate::engine::Objects::Sphere::Sphere;
use crate::engine::SHADERS::shader_manager as sm;
use crate::engine::PError::PError;
use crate::engine::PArc::PArc;
use crate::engine::Objects::ObjectManagement::ObjectStorage::*;
use pyo3::{Py};
use pyo3::types::{PyWeakref};

use crate::engine::Objects::Cube::*;
use crate::engine::Objects::ObjectManagement::ObjectStorage;
use crate::engine::Objects::ObjectManagement::ObjectManagement;


pub enum Command {
    ManuallyStepPhysics(f32),
    EnableCollisionForObject(DefaultKey),
    DisableCollisionForObject(DefaultKey),
    GetColissionObjects{
        key: DefaultKey, sender: mpsc::SyncSender<Vec<Arc<Py<PyWeakref>>>>,
    },
    DrawAll3DObjects(),

    DeleteObject{
        key: DefaultKey, 
    },
    GetObjectScale{ key: DefaultKey, sender: mpsc::SyncSender<mq::Vec3> },
    GetObjectPos{ key: DefaultKey, sender: mpsc::SyncSender<mq::Vec3> },
    GetObjectRotation{ key: DefaultKey, sender: mpsc::SyncSender<mq::Vec3> },

    SetObjectScale{ key: DefaultKey, scale: mq::Vec3 },
    SetObjectPos{ key: DefaultKey, position: mq::Vec3 },
    SetObjectRotation{ key: DefaultKey, rotation: mq::Vec3 },

    CreateCube{
        size: mq::Vec3,
        position: mq::Vec3,
        rotation: mq::Vec3,
        color: mq::Color,
        weak_ref: Py<PyWeakref>,
        sender: mpsc::SyncSender<DefaultKey>,
    },

    CreateMesh{
        mesh: Mesh,
        weak_ref: Py<PyWeakref>,
        sender: mpsc::SyncSender<DefaultKey>,
    },

    CreateSphere{
        size: mq::Vec3,
        position: mq::Vec3,
        rotation: mq::Vec3,
        color: mq::Color,
        weak_ref: Py<PyWeakref>,
        sender: mpsc::SyncSender<DefaultKey>,
    },

    DropThisItem(Arc<dyn Any + Send + Sync>), // drops it's item.  >_<

    LoadFile{ path: String, sender: mpsc::SyncSender<Result<Vec<u8>, PError>> },
    LoadSound{ path: String, sender: mpsc::SyncSender<Result<PArc<au::Sound>, PError>> },
    
    LoadSoundFromBytes{data: Vec<u8>, sender: mpsc::SyncSender<Result<PArc<au::Sound>, PError>> },

    PlaySound{ sound: au::Sound, params: au::PlaySoundParams },

    PlaySoundOnce{ sound: au::Sound },

    SetSoundVolume{ sound: au::Sound, volume: f32 },

    StopSound{ sound: au::Sound },

    RenderTargetMsaa{ width: u32, height: u32, sender: mpsc::SyncSender< PArc<mq::RenderTarget>  > },
    RenderTargetEx{ width: u32, height: u32, params: Option<mq::RenderTargetParams>, sender: mpsc::SyncSender<PArc<mq::RenderTarget> > },
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


    GetMousePosition(mpsc::SyncSender<(f32,f32)>),
    
    GetKeysPressed(mpsc::SyncSender<HashSet<mq::KeyCode>>),

    GetKeysDown(mpsc::SyncSender<HashSet<mq::KeyCode>>),

    GetKeysReleased(mpsc::SyncSender<HashSet<mq::KeyCode>>),

    NextFrame{physics_step: Option<f32>, sender: mpsc::SyncSender<()>},

    ImgToTexture {
        image: Arc<mq::Image>,
        sender: mpsc::SyncSender<PArc<mq::Texture2D>>,
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
    pub static ref COMMAND_QUEUE: Arc<SegQueue<Command>> = Arc::new(SegQueue::new());
    
}

lazy_static! {
    pub static ref ComputationTime: Arc<Vec<u128>> = Arc::new(Vec::new());

}

/// processes commands that rely on the macroquad engine
/// commands that do not rely on it's core (openGL) components ( or just the internal Core-Thread ) are found in pyabstractions.
pub async fn proccess_commands_loop() {

    let mut object_storage = ObjectStorage::ObjectStorage::new();

    loop {
        while let Some(command) = COMMAND_QUEUE.pop() {
            
            match command {
                Command::DisableCollisionForObject(key)=> {
                    object_storage.remove_collision_for_object(key);
                }
                Command::EnableCollisionForObject(key)=> {
                    object_storage.add_collision_for_object(key);
                }
                Command::GetColissionObjects { key, sender }=>{
                    let keys = object_storage.collides_with(key);
                    let py_refs  = object_storage.keys_to_py(keys);
                    let _ = sender.send(py_refs);
                }
                Command::ManuallyStepPhysics(distance)=>{
                    object_storage.step_physics(distance);
                }
                Command::DrawAll3DObjects()=> {
                    let matrix;
                    unsafe {
                        let mat = mq::get_internal_gl();
                        matrix= mat.quad_gl.get_projection_matrix()
                    }
                    sm::switch_to_desired_shader(sm::ShaderKind::Basic);
                    ObjectManagement::draw_all_Objects(&object_storage, matrix);
                }
                Command::DeleteObject { key }=> {
                    object_storage.remove_object(key);
                }
                Command::GetObjectPos { key, sender } => {
                    let pos = match  object_storage.get(key){
                        Object::Cube(cube) => cube.position,
                        Object::Mesh(mesh) => mesh.position,
                        _ => todo!(),
                    };
                    let _ = sender.send(pos);
                }
                Command::GetObjectScale { key, sender } => {
                    let pos = match  object_storage.get(key){
                        Object::Cube(cube) => cube.scale,
                        Object::Mesh(mesh)=> mesh.scale,
                        _ => todo!(),
                    };
                    let _ = sender.send(pos);
                }
                Command::GetObjectRotation { key, sender } => {
                    let pos = match  object_storage.get(key){
                        Object::Cube(cube) => cube.rotation,
                        Object::Mesh(mesh)=> mesh.rotation,
                        _ => todo!(),
                    };
                    let _ = sender.send(pos);
                }
                Command::SetObjectPos { key, position } => {
                    object_storage.change_obj_position(&position, key, 
                        move |obj|{
                        match obj{
                            Object::Cube(cube)=> {
                                cube.mesh.recalculate_pos(cube.position, position);
                                cube.position = position;
                            }
                            Object::Mesh(mesh) => {
                                mesh.recalculate_pos(mesh.position, position);
                                mesh.position =  position;
                            }
                            _ => todo!()
                        }
                    });
                }
                Command::SetObjectScale { key, scale } => {

                    object_storage.change_obj_scale(&scale, key, 
                        move |obj|{
                            match obj{
                                Object::Cube(cube)=> {
                                    cube.mesh.recalculate_scale(cube.position, cube.scale, scale);
                                    cube.scale = scale;
                                }
                                Object::Mesh(mesh) => {
                                    mesh.recalculate_scale(mesh.position,mesh.scale, scale);
                                    mesh.scale =  scale;
                                }
                                _ => todo!()
                            }
                        });
                }
                Command::SetObjectRotation { key, rotation } => {
                    object_storage.change_obj_rotation(&rotation, key, 
                        move |obj|{
                            match obj{
                                Object::Cube(cube)=> {
                                    cube.mesh.recalculate_rot(cube.position, cube.rotation, rotation);
                                    cube.rotation = rotation;
                                }
                                Object::Mesh(mesh) => {
                                    mesh.recalculate_rot(mesh.position, mesh.rotation, rotation);
                                    mesh.rotation =  rotation;
                                    

                                }
                                _ => todo!()
                            }
                        });
                    
                }
                Command::CreateCube { size, position, rotation,color, weak_ref: pyAny, sender }=>{

                    object_storage.quick_push(sender, pyAny, 
                        move || {
                            let internal_cube = Cube::new(size, position, rotation, color);
                            Object::Cube(internal_cube)
                        });
                        
                }
                Command::CreateMesh { mesh, weak_ref, sender }=>{

                    object_storage.quick_push(sender, weak_ref, 
                        move || {
                            Object::Mesh(mesh)
                        });
                }
                Command::CreateSphere { size, position, rotation,color, weak_ref: pyAny, sender }=>{

                    object_storage.quick_push(sender, pyAny, 
                        move || {
                            let internal_sphere = Sphere::new(size, position, rotation, color);
                            Object::Sphere(internal_sphere)
                        });
                        
                }
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
                Command::SetDefaultCamera() =>{ 
                    mq::set_default_camera() 
                }
        
                Command::DrawTexture { texture,x,y,color}=>
                {
                    sm::switch_to_desired_shader(sm::ShaderKind::None);
                    mq::draw_texture(&texture,x,y,color)
                }
                
                Command::ClearBackground {color } => {
                    macroquad::prelude::clear_background(color);
                }
        
                Command::NextFrame{physics_step, sender} => {
                    mq::next_frame().await;
                    crate::engine::SHADERS::shader_manager::new_frame_shader_update();
                    crate::engine::FrameInfo::update_frame_info();
                    let _ = sender.send(());
                    
                    if physics_step.is_some(){
                        object_storage.step_physics(physics_step.unwrap());
                    }
                }
            
                Command::DrawText { text, x, y, font_size, color }=>{
                sm::switch_to_desired_shader(sm::ShaderKind::None);
                mq::draw_text(text.as_str(), x,y,font_size,color);
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
                    let tex: PArc<mq::Texture2D> = PArc::new(  mq::Texture2D::from_image(&image));
                    let _ = sender.send(tex);
                }
                Command::SetCamera { camera_2d, camera_3d } => { // merged cam2d and 3d for simplicity.
                    match (camera_2d, camera_3d) {
                        (Some(cam), None) => {
                            mq::set_camera(&cam);
                        },
                        (None, Some(cam)) => {
                            mq::set_camera(&cam);
                        },
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
                    let _ = sender.send(PArc::new(render_target));
                }
                Command::RenderTargetEx{ width, height, params, sender } => {
                    match params{
                        Some(p) => {
                            let render_target = mq::render_target_ex(width, height, p);
                            let _ = sender.send(PArc::new(render_target));
                        }
                        None => {
                            let render_target = mq::render_target_ex(width, height, mq::RenderTargetParams::default());
                        let _ = sender.send(PArc::new(render_target));
                        }
                    }
        
                }
                Command::LoadSound { path ,sender} => {
                    let result = async {
                        let data = macroquad::prelude::load_file(&path).await?;
        
                        //converts .mp3 to .wav
                        let secured_data  = crate::engine::AudioConverter::ensure_wav(data).map_err(|e| {
                        e.with_context(format!(" path: {path}"))
                        }  )?;
        
                        let sound = au::load_sound_from_bytes(&secured_data).await?;
                        Ok(sound)
                    }.await;
                    let result = result.map(|op| PArc::new(op)  );
                
                    let _ = sender.send(result);
                }
                Command::LoadSoundFromBytes { data ,sender} => {
        
                    let result: Result<_, PError> = async {
                        //converts .mp3 to .wav
                        let secured_data  = crate::engine::AudioConverter::ensure_wav(data)?;
        
                        let sound = au::load_sound_from_bytes(&secured_data).await?;
                        Ok(sound)
                    }.await;
                
                    let result = result.map(|op| PArc::new(op)  );
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
        
                Command::DropThisItem(item_arc)=>{}
        
                
            }
        }


    }
}