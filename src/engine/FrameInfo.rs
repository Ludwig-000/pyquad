use std::sync::Mutex;
use std::sync::atomic::{AtomicI32,AtomicBool,AtomicU32,Ordering,AtomicU64};
use macroquad::prelude as mq;
use std::time::Instant;
use std::sync::LazyLock;

pub static DELTA_TIME: Mutex<f32>  = Mutex::new(0.0);

pub static FPS: AtomicI32 =  AtomicI32::new(0);

/// this function should be run my 'next_frame'
pub fn update_frame_info(){
    let fps =  mq::get_fps();
    FPS.store(fps, Ordering::Relaxed);

    let dt = mq::get_frame_time();
    *DELTA_TIME.lock().unwrap() = dt;
}