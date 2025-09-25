use macroquad::prelude::*;
use std::sync::atomic::{AtomicU8, Ordering,AtomicUsize};
use std::sync::Mutex;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ShaderKind {
    None = 0,
    Basic = 1,
}
impl From<u8> for ShaderKind {
    fn from(v: u8) -> Self {
        match v {
            
            1 => ShaderKind::Basic,
            _ => ShaderKind::None,
        }
    }
}
impl From<ShaderKind> for u8 {
    fn from(k: ShaderKind) -> Self {
        k as u8
    }
}


static CURRENT_SHADER: AtomicU8 = AtomicU8::new(ShaderKind::None as u8);
static SWITCH_COUNT: AtomicUsize = AtomicUsize::new(0);

lazy_static::lazy_static! {
    static ref SWITCH_HISTORY: Mutex<Vec<usize>> = Mutex::new(Vec::new());
}


#[inline(always)]
fn get_current_shader() -> ShaderKind {
    ShaderKind::from(CURRENT_SHADER.load(Ordering::Relaxed))
}

#[inline(always)]
fn set_current_shader(desired: ShaderKind) {
    CURRENT_SHADER.store(desired.into(), Ordering::Relaxed);
}

pub fn switch_to_desired_shader(desired: ShaderKind) {
    let current = get_current_shader();
    if current == desired {
        return;
    }
    set_current_shader(desired);
    // Increment switch counter atomically
    SWITCH_COUNT.fetch_add(1, Ordering::Relaxed);
    // Defer actual GPU shader bind to rendering loop
    load_shader(desired);
}


// Call this at the start of every new frame to record switch count and reset
pub fn new_frame_shader_update() {

    set_current_shader(ShaderKind::None); // newframe resets all shaders idk why.

    let switches = SWITCH_COUNT.swap(0, Ordering::Relaxed);

    let mut history = SWITCH_HISTORY.lock().unwrap();
    history.push(switches);

    // analyze the shader history
    let frameCount=  history.len();
    if frameCount < 100 { return; }

    let mut average_switches: usize =0;
    for  &i in history.iter(){
        average_switches+= i;
    }
    average_switches /= frameCount;

    if average_switches >= 100 { 
        warn!("We detected an average of {} shader switches! This may impace performance. Try batching draw calls with the same shader.",average_switches);
    }

    history.clear();
}


fn get_switch_history() -> Vec<usize> {
    SWITCH_HISTORY.lock().unwrap().clone()
}




fn load_shader(shader: ShaderKind){
   match shader {
    ShaderKind::None => {
        gl_use_default_material();
    }
    ShaderKind::Basic => {
        let material = crate::engine::SHADERS::shaderLoader::get_shader(0).expect("Basic shader not loaded");
        macroquad::material::gl_use_material(&material);
    }
   }
    
}