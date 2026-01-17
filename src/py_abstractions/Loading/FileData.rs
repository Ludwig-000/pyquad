
use pyo3::prelude::*;
use pyo3_stub_gen::derive::* ;
use crate::py_abstractions::structs::{Audio::Sound, Textures_and_Images::{Image, Texture2D}};



/// A wrapper around raw filedata that has not yet been parsed.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct FileData{

    /// These are the raw bytes of a file.
    /// They are not intended to be edited manually.
    #[pyo3(get,set)]
    pub bytes: Vec<u8>,
}

#[gen_stub_pymethods]
#[pymethods]
impl FileData{

    #[staticmethod]
    pub fn from_bytes(bytes: Vec<u8>)-> Self{
        FileData { bytes}
    }
    
    #[staticmethod]
    pub fn into_Image()-> Image{
        todo!()
    }

    /// Attempts to parse the file data into a texture.
    /// FileData -> Image -> Texture2D
    /// 
    #[staticmethod]
    pub fn into_2DTexture()-> Texture2D{
        todo!()
    }

    #[staticmethod]
    pub fn into_Sound()-> Sound{
        todo!()
    }

    #[staticmethod]
    pub fn into_mesh_data(){
        todo!()
    }
    
}
