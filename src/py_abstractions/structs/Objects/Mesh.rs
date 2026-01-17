use macroquad::prelude::Vertex as mq_vert;
use pyo3::{pyclass, pymethods};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

use slotmap::DefaultKey;
use slotmap::Key;

use crate::engine::PError::PError;
use crate::py_abstractions::{Loading::FileData::FileData, structs::Textures_and_Images::Texture2D};
use crate::py_abstractions::structs::GLAM::Vec3::Vec3;
use crate::engine::Objects::Mesh as internal_mesh;
use pyo3::prelude::PyResult;
use crate::engine::CoreLoop::{Command,COMMAND_QUEUE};
use pyo3::prelude::*;
use std::sync::mpsc;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::* ;
use pyo3::types::{PyWeakref, PyWeakrefReference};

#[gen_stub_pyclass]
#[pyclass]
pub struct Mesh{
	pub key: DefaultKey
}

#[gen_stub_pymethods]
#[pymethods]
impl Mesh{
    #[new]
    fn new()-> Self{
        todo!()
    }

    #[staticmethod]
    pub fn from_file_data(py: Python<'_>,data: FileData, position: Vec3, rotation: Vec3, scale: Vec3)-> PyResult<Py<Mesh>>{
        let mesh =  internal_mesh::Mesh::load_from_gltf(&data.bytes, None).map_err(|e|{
            PError::GLTFError(e)
        })?;

        
        let (sender, receiver) = mpsc::sync_channel(1);

        let placeholder_struct: Mesh = Mesh { key: DefaultKey::null()  };
        let mesh_handle: Py<Mesh> = Py::new(py, placeholder_struct)?; 
        
        let weak_ref_handle: Py<PyWeakref> = {
            let bound_cube = mesh_handle.bind(py); 
            let weak_ref_ref = PyWeakrefReference::new(&bound_cube)?;
            weak_ref_ref.cast_into::<PyWeakref>()?.unbind() 
        };


        COMMAND_QUEUE.push(Command::CreateMesh { mesh, weak_ref: weak_ref_handle, sender });
        
        let key: DefaultKey = receiver.recv().unwrap();
        
        mesh_handle.borrow_mut(py).key = key;

        Ok(mesh_handle) 
    }
}
