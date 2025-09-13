use pyo3::prelude::*;
use pyo3::wrap_pyfunction; 
use macroquad::prelude as mq;
use std::sync::Arc;
use crate::py_abstractions::py_structs::*;
use std::option;
use std::sync::mpsc;
use crate::COMMAND_QUEUE;
use crate::Command;
use pyo3::exceptions::PyValueError;
use crate::py_abstractions::structs::GLAM::Vec3::Vec3;
use crate::py_abstractions::structs::GLAM::Vec2::Vec2;
#[pyclass(name = "Camera2D")]
#[derive(Debug,Clone)]
pub struct Camera2D {
    /// Rotation in degrees.
    #[pyo3(get, set)]
    pub rotation: f32,
    /// Scaling, should be (1.0, 1.0) by default.
    #[pyo3(get, set)]
    pub zoom: Vec2,
    /// Rotation and zoom origin.
    #[pyo3(get, set)]
    pub target: Vec2,
    /// Displacement from target.
    #[pyo3(get, set)]
    pub offset: Vec2,

    #[pyo3(get, set)]
    pub viewport: Option<(i32, i32, i32, i32)>,
}

   
#[pymethods]  
impl Camera2D {
    #[new]
    fn new() -> Self {
    Camera2D::default()
    }
    /// Will make camera space equals given rect.
    #[staticmethod]
    pub fn from_display_rect(rect: Rect) -> Camera2D {
        let target = Vec2{ x: rect.x + rect.w / 2.,y: rect.y + rect.h / 2.  };

        Camera2D {
            target,
            zoom: Vec2{x: 1. / rect.w * 2.,y: -1. / rect.h * 2.},
            offset: Vec2{x: 0.,y: 0.},
            rotation: 0.,
            viewport: None,
        }
    }

    #[staticmethod]
    /// Set active 2D or 3D camera.
    pub fn set_camera(camera: Camera2D) {

        let cam: mq::Camera2D = mq::Camera2D{  rotation: camera.rotation, zoom: mq::vec2(camera.zoom.x,camera.zoom.y),target: mq::vec2(camera.target.x,camera.target.y)  ,offset: mq::vec2(camera.offset.x,camera.offset.y),render_target: None,viewport: None};

        COMMAND_QUEUE.push(Command::SetCamera { camera_2d: Some(cam), camera_3d: None });


    }
}

impl Default for Camera2D {
    fn default() -> Camera2D {
        Camera2D {
            zoom: Vec2{x: 1.,y: 1.},
            offset: Vec2{x: 0.,y: 0.},
            target: Vec2{x: 0.,y: 0.},
            rotation: 0.,
            viewport: None,
        }
    }

}





//impl From<Camera2D> for mq::Camera2D {
//    fn from(v: Camera2D) -> Self {
//        mq::Camera2D {
//            rotation: v.rotation,
//            zoom: v.zoom,target: v.target,offset: v.offset,render_target: v.render_target,viewport: v.viewport,
//        }
//    }
//}


//impl From<mq::Camera2D> for Camera2D {
//    fn from(v: mq::Camera2D) -> Self {
//        Camera2D {
//        rotation,zoom,offset,target: v.render_target,viewport,
//        }
//    }
//}


#[pyfunction]
pub fn set_default_camera()  {
    COMMAND_QUEUE.push(Command::SetDefaultCamera());

}





#[pyclass(name = "Camera3D")]
#[derive(Debug,Clone)]
pub struct Camera3D {
    #[pyo3(get, set)]
    pub position: Vec3,
    #[pyo3(get, set)]
    pub target: Vec3,
    #[pyo3(get, set)]
    pub up: Vec3,
    #[pyo3(get, set)]
    pub fovy: f32,
    /// Screen aspect ratio.
    ///
    /// By default aspect is calculated with screen_width() / screen_height() on each frame.
    #[pyo3(get, set)]
    pub aspect: Option<f32>,

    /// Camera near plane
    #[pyo3(get, set)]
    pub z_near: f32,
    /// Camera far plane
    #[pyo3(get, set)]
    pub z_far: f32,
}

   
#[pymethods]  
impl Camera3D {
    #[new]
    fn new() -> Self {
    Camera3D::default()
    }

    #[staticmethod]
    /// Set active 2D or 3D camera.
    pub fn set_camera(camera: Camera3D) {
        let cam = mq::Camera3D {
            position: mq::vec3(camera.position.x, camera.position.y, camera.position.z),
            target: mq::vec3(camera.target.x, camera.target.y, camera.target.z),
            up: mq::vec3(camera.up.x, camera.up.y, camera.up.z),
            fovy: camera.fovy,
            aspect: camera.aspect,
            projection: mq::Projection::Perspective,
            viewport: None,
            render_target: None,
            z_near: camera.z_near,
            z_far: camera.z_far,
        };

        COMMAND_QUEUE.push(Command::SetCamera { camera_2d: None, camera_3d: Some(cam) });
    }
}
impl Default for Camera3D {
    fn default() -> Camera3D {
        Camera3D {
            position: Vec3{x: 0.,y: -10.,z: 0.},
            target: Vec3{x:0.,y: 0.,z: 0.},
            aspect: None,
            up: Vec3{x:0.,y: 0.,z: 1.},
            fovy: 45.0_f32.to_radians(),
            z_near: 0.01,
            z_far: 10000.0,
        }
    
    }

}


