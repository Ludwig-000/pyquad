
use pyo3::prelude::*;
use pyo3_stub_gen::derive::* ;
use crate::py_abstractions::structs::{Audio::Sound, Textures_and_Images::{Image, Texture2D}};



/// A wrapper around raw filedata.
#[gen_stub_pyclass]
#[pyclass]
pub struct Filedata{
    #[pyo3(get,set)]
    pub bytes: Vec<u8>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Filedata{

    #[staticmethod]
    pub fn from_bytes(bytes: Vec<u8>)-> Self{
        Filedata { bytes}
    }
    
    #[staticmethod]
    pub fn into_Image()-> Image{
        todo!()
    }

    #[staticmethod]
    pub fn into_2DTexture()-> Texture2D{
        todo!()
    }

    #[staticmethod]
    pub fn into_Sound()-> Sound{
        todo!()
    }
    
}
