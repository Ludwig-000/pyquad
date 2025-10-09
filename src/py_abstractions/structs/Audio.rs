use macroquad::miniquad::StencilFaceState;
use macroquad::prelude as mq;
use macroquad::audio as au;
use pyo3::{pyclass, pyfunction,pymethods};
use pyo3_stub_gen::derive::gen_stub_pyfunction;
use crate::py_abstractions::structs::Audio;
use crate::py_abstractions::structs::Textures_and_Images::Texture2D;
use std::sync::mpsc;
use pyo3_stub_gen::{define_stub_info_gatherer,derive::*};
use crate::COMMAND_QUEUE;
use crate::Command;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction; 

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone, Debug)]
pub struct Sound {
   audio: au::Sound,
}



#[gen_stub_pymethods]
#[pymethods]
impl Sound {

    /// Load audio file.
    ///
    /// Attempts to automatically detect the format of the source of data.
    /// 
    /// supported filetypes: ".wav", ".mp3"
    #[staticmethod]
    pub fn load_sound(path: String)-> PyResult<Sound>{

        let (sender, receiver) = mpsc::sync_channel(1);

        COMMAND_QUEUE.push( Command::LoadSound { path: path, sender } );

        match receiver.recv() {
            Ok(sound) => {
                match sound{
                    Ok(s) => { Ok(s.into())  },
                    Err(e) => {
                        Err(e.into())
                    }
                }
            }
            Err(e) => panic!("Fatal MSPC Error:  {e}"),
        }
    }

    /// Load audio data.
    ///
    /// Attempts to automatically detect the format of the source of data.
    #[staticmethod]
    pub fn load_sound_from_bytes(data: Vec<u8>)-> PyResult<Sound> {

        let (sender, receiver) = mpsc::sync_channel(1);
        
        COMMAND_QUEUE.push( Command::LoadSoundFromBytes { data: data, sender} );

        match receiver.recv() {
            Ok(sound) => {
                match sound{
                    Ok(s) => { Ok(s.into())  },
                    Err(e) => {
                        Err(e.into())
                        
                    }
                }
            }
            Err(e) => panic!("Fatal MSPC Error:  {e}"),
        }
    }

    pub fn play_sound(&self, params: PlaySoundParams){ 
        COMMAND_QUEUE.push( Command::PlaySound { sound: self.audio.clone() , params: params.into() }  );
    }

    pub fn play_sound_once(&self){ 
        COMMAND_QUEUE.push( Command::PlaySoundOnce { sound: self.audio.clone()  } );
    }

    pub fn set_sound_volume(&self, volume: f32){ 
        
        COMMAND_QUEUE.push( Command::SetSoundVolume { sound: self.audio.clone() , volume: volume } );
    }

    pub fn stop_sound(&self){ 
        COMMAND_QUEUE.push( Command::StopSound { sound: self.audio.clone() } );
    }

}



impl From<au::Sound> for Sound{
    fn from(s: au::Sound) -> Self {
        Sound{ audio: s }
    }
}


impl From<Sound> for au::Sound{
    fn from(s: Sound) -> Self {
        s.audio
    }
}



#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone, Copy)]
pub struct PlaySoundParams {

    #[pyo3(get, set)]
    pub looped: bool,

    #[pyo3(get, set)]
    pub volume: f32,
}

#[gen_stub_pymethods]
#[pymethods]
impl PlaySoundParams {

    #[new]
    #[pyo3(signature = ( looped = false, volume = 1.))]
    pub fn new(looped: bool, volume: f32) -> Self {
        Self { looped, volume }
    }

}


impl From<au::PlaySoundParams> for PlaySoundParams{
    fn from(r: au::PlaySoundParams) -> Self {
        PlaySoundParams { looped: r.looped, volume: r.volume }
    }
}



impl From<PlaySoundParams> for au::PlaySoundParams{
    fn from(r: PlaySoundParams) -> Self {
        au::PlaySoundParams { looped: r.looped, volume: r.volume }
    }
}
