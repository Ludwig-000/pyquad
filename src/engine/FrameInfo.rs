use std::sync::Mutex;
use std::sync::atomic::{AtomicI32,AtomicBool,AtomicU32,Ordering,AtomicU64};
use macroquad::prelude as mq;
use std::time::Instant;
use std::sync::LazyLock;


/// Some information about the state of the engine, that is only updates once a frame,
/// meaning it can be stored safely inside statics.
pub static DELTA_TIME: Mutex<f32>  = Mutex::new(0.0);
pub static FPS: AtomicI32 =  AtomicI32::new(0);


/// this function should be run by 'next_frame'
pub fn update_frame_info(){
    let fps =  mq::get_fps();
    FPS.store(fps, Ordering::Relaxed);

    let dt = mq::get_frame_time();
    *DELTA_TIME.lock().unwrap() = dt;
}