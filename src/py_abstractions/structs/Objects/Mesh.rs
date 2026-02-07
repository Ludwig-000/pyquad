use macroquad::prelude::Vertex as mq_vert;
use pyo3::ffi::PyCallable_Check;
use pyo3::{pyclass, pymethods};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

use pyo3_stub_gen::inventory::submit;
use slotmap::DefaultKey;
use slotmap::Key;
use std::hash::{Hash, Hasher};
use crate::basic_3D_magic_methods;
use crate::engine::Objects::ObjectDataCache::ThreeDObjCache;
use crate::engine::PError::PError;
use crate::py_abstractions::structs::Objects::ColliderOptions::ColliderOptions;
use crate::py_abstractions::{Loading::FileData::FileData, structs::Textures_and_Images::Texture2D};
use crate::py_abstractions::structs::GLAM::Vec3::Vec3;
use crate::engine::Objects::Mesh as internal_mesh;
use pyo3::prelude::PyResult;
use crate::engine::CoreLoop::{Command,COMMAND_QUEUE};
use pyo3::prelude::*;

use crate::engine::PChannel::PChannel;

use pyo3::prelude::*;
use pyo3_stub_gen::derive::* ;
use pyo3::types::{PyWeakref, PyWeakrefReference};
use crate::py_abstractions::structs::Objects::ObjectFunctionStorage::FunctionKey;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::* ;

use pyo3::exceptions::*;



use crate::engine::Objects::ObjectDataCache;

use crate::py_abstractions::structs::Objects::ObjectFunctionStorage;
use crate::py_abstractions::Color::Color;
use crate::engine::Objects::ObjectManagement::ObjectStorage::ObjectKey;
use crate::py_abstractions::structs::Objects::ObjectMacros::*;



#[gen_stub_pyclass]
#[pyclass(subclass, weakref)]
pub struct Mesh{
	pub key: ObjectKey,
    pub function_key: Option<FunctionKey>,

    // an object's data can only be cached, if it can NOT be influenced by anything external.
    // F.E.: gravity.
    pub cache: Option<ObjectDataCache::ThreeDObjCache>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Mesh{

    #[staticmethod]
    pub fn from_file_data(py: Python<'_>,data: FileData, collider_type: ColliderOptions)-> PyResult<Py<Mesh>>{
        
        let (sender, receiver) = PChannel::sync_channel(1);

        let placeholder_struct: Mesh = Mesh { key: ObjectKey::null(), 
            function_key: None, 
            cache: None,
        };
        let mesh_handle: Py<Mesh> = Py::new(py, placeholder_struct)?; 
        
        let weak_ref_handle: Py<PyWeakref> = {
            let bound_mesh = mesh_handle.bind(py); 
            let weak_ref_ref = PyWeakrefReference::new(&bound_mesh)?;
            weak_ref_ref.cast_into::<PyWeakref>()?.unbind() 
        };

        let mesh =  internal_mesh::Mesh::load_from_gltf(&data.bytes, None).map_err(|e|{
            PError::GLTFError(e)
        })?;

        COMMAND_QUEUE.push(Command::CreateMesh { mesh, collider: collider_type, weak_ref: weak_ref_handle, sender });
        
        let key = receiver.recv()?;
        
        mesh_handle.borrow_mut(py).key = key;

        Ok(mesh_handle) 
    }


    #[getter]
    fn scale(&self) -> PyResult<Vec3> {
        if let Some(cache) = self.cache {
            return Ok(cache.scale.into())
        }

        let (sender, receiver) = PChannel::sync_channel(1);
        let command = Command::GetObjectScale { key: self.key, sender: sender };
        COMMAND_QUEUE.push(command);
        Ok(receiver.recv()?.into())
    }

    #[setter]
    fn set_scale(&mut self, value: Vec3) {
        if let Some(cache) = &mut self.cache {
            cache.scale = value.into();
        }

        let command = Command::SetObjectScale { key: self.key, scale: value.into() };
        COMMAND_QUEUE.push(command);
    }

    #[getter]
    fn pos(&self) -> PyResult<Vec3> {
        if let Some(cache) = self.cache {
            return Ok(cache.position.into())
        }

        let (sender, receiver) = PChannel::sync_channel(1);
        let command = Command::GetObjectPos { key: self.key, sender: sender };
        COMMAND_QUEUE.push(command);
        Ok(receiver.recv()?.into())
    }

    #[setter]
    fn set_pos(&mut self, value: Vec3) {
        if let Some(cache) = &mut self.cache {
            cache.position = value.into();
        }
        let command = Command::SetObjectPos { key: self.key, position: value.into() };
        COMMAND_QUEUE.push(command);
    }

    #[getter]
    fn rot(&self) -> PyResult<Vec3> {
        if let Some(cache) = self.cache {
            return Ok(cache.rotation.into())
        }

        let (sender, receiver) = PChannel::sync_channel(1);
        let command = Command::GetObjectRotation{ key: self.key, sender: sender };
        COMMAND_QUEUE.push(command);
        Ok(receiver.recv()?.into())
    }

    #[setter]
    fn set_rot(&mut self, value: Vec3) {
        if let Some(cache) = &mut self.cache {
            cache.rotation = value.into();
        }

        let command = Command::SetObjectRotation { key: self.key, rotation: value.into() };
        COMMAND_QUEUE.push(command);
    }

    pub fn set_collision(&self, collision_type: ColliderOptions){
        let command = Command::SetCollisionForObject{key: self.key, collider: collision_type};
        COMMAND_QUEUE.push(command);
    }

    /// Returns any object, with active collision, that is either
    /// intersected or inserted in the current object.
    /// 
    ///Example:
    /// 
    ///```
    ///>>>bigCube: Cube = Cube(pos=Vec3.splat(50))
    ///>>>intersected: list[Cube] = bigCube.check_collision()
    ///...
    ///...# since the returned objects are references, we can edit them directly
    ///...# without creating duplicates.
    ///>>>for cube in intersected:
    ///...   cube.pos = Vec3.ZERO()
    ///```
    pub fn check_collision<'py>(&self, py: Python<'py> )-> PyResult<Vec<Bound<'py, PyAny>>>{

        let (sender, receiver) = PChannel::sync_channel(1);
        let command = Command::GetColissionObjects { key: self.key, sender };
        COMMAND_QUEUE.push(command);
        let res: Vec<std::sync::Arc<Py<PyWeakref>>> = receiver.recv()?;

        // we filter map, since any weakref we hold may already be invalid.
        Ok(res.into_iter().filter_map(|pyObj|{

            let weak_py: &Py<PyWeakref> = &*pyObj;

            let weak_bound: Bound<'py, PyWeakref> = weak_py.bind(py).clone();

            let upgraded: Bound<'py, PyAny> = weak_bound.call0().ok()?;

            if upgraded.is_none() {
                None
            } else {
                Some(upgraded)
            }
        }).collect())
    }

    /// Add a function to this object, which will automatically be executed each frame.
    /// The function must take the object it is attatched to as an argument.
    /// 
    ///Example:
    /// 
    ///```
    ///...# arguments from outside the scope may be included.
    ///>>>delta_time = 0
    ///>>>def updateCube(cube: Cube):
    ///...    cube.rot += Vec3.splat(0.2*delta_time)
    ///...
    ///>>>myCube = Cube()
    ///>>>myCube.tick(updateCube)
    ///...
    ///>>>while True:
    ///...    # dt would have to get updated each frame.
    ///...    delta_time = get_delta_time()
    ///...
    ///...    #'next_frame' runs the update function for every object.
    ///...    next_frame()
    /// ```
    /// 
    pub fn tick(slf: Bound<'_, Self>, function: Bound<'_,PyAny>)-> PyResult<()>{
        if !function.is_callable(){
            return Err(PyRuntimeError::new_err(format!("Attatched object {:?} is not callable.",function)));
        }

        let mut storage = ObjectFunctionStorage::get_fun_storage();
        
        let func_persistent = function.unbind();
        let obj  = slf.into_any();

        let key = storage.add(obj, func_persistent);

        Ok(())
    }

    pub fn remove_tick(&mut self)-> PyResult<()>{

        let mut storage = ObjectFunctionStorage::get_fun_storage();
        let key = match self.function_key{
            None => { 
                return Err(
                    PyRuntimeError::new_err("No function found, that can be removed.")
                );
            },
            Some(key)=> { key },
        };
        storage.remove(key);
        self.function_key  = None;
        Ok(())
    }

    pub fn bind_location(&mut self, obj: Bound<'_, PyAny>){
        todo!()
    }
    pub fn unbind_location(&mut self){
        todo!()
    }
    pub fn bind_rotation(&mut self){
        todo!()
    }
    pub fn unbind_rotation(&mut self){
        todo!()
    }
    pub fn bind_scale(&mut self){
        todo!()
    }
    pub fn unbind_scale(&mut self){
        todo!()
    }

}
basic_3D_magic_methods!(Mesh);

impl Drop for Mesh{
    fn drop(&mut self) {

        // function storage MUST be cleaned first, since a function inside fun-storage may rely on the object still living.
        match self.function_key{
            None => {},
            Some(key)=> {
                let mut storage = ObjectFunctionStorage::get_fun_storage();
                storage.remove(key);
            }
        }
        COMMAND_QUEUE.push( Command::DeleteObject { key: self.key });
    }
}