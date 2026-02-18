/// All internal Camera Fnctions MUST pass through this file.
/// 

use macroquad::{camera::{Camera2D, Camera3D}, prelude as mq};


// because for some reason, macroquad's cameras don't implement clone
pub fn clone_camera3d(cam: &Camera3D)-> Camera3D{
    mq::Camera3D{
        position: cam.position,
        target: cam.target,
        up: cam.up,
        fovy: cam.fovy,
        aspect: cam.aspect,
        projection: cam.projection,
        render_target: cam.render_target.clone(),
        viewport: cam.viewport,
        z_near: cam.z_near,
        z_far: cam.z_far
    }
}

// because for some reason, macroquad's cameras don't implement clone
pub fn clone_camera2d(cam: &Camera2D)-> Camera2D{
    mq::Camera2D{
        rotation: cam.rotation,
        zoom: cam.zoom,
        target: cam.target,
        offset: cam.offset,
        render_target: cam.render_target.clone(),
        viewport: cam.viewport,
    }
}

pub struct CamMemory{
    pub current_cam: Camera,
    pub last_3d_cam: Option<mq::Camera3D>,
    pub last_2d_camera: Option<mq::Camera2D>
}

pub fn set_camera(camera: &dyn mq::Camera){
    #[allow(clippy::disallowed_methods)]
    mq::set_camera(camera);
}
pub enum Camera{
    Camera2D(mq::Camera2D),
    Camera3D(mq::Camera3D)
}