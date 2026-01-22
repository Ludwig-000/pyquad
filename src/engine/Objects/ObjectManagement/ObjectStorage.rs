use crate::engine::{Objects::{Cube::Cube, Mesh::Mesh, Sphere::Sphere}, structures::Rectangle};
use pyo3::prelude::*;
use pyo3::types::PyWeakref;
use slotmap::*;
use std::{sync::Arc};
use crate::engine::Objects::PhysicsWorld::Rapier::{ObjectHandle, RapierWorld};
use std::sync::mpsc::SyncSender;
use macroquad::prelude as mq;

pub enum Object {
    Cube(Cube),
    Sphere(Sphere),
    Mesh(Mesh)
}

#[derive(Clone, Copy)]
pub struct GlueData{
    pub reverse_lookup: DefaultKey,
    pub obj_handle:  ObjectHandle,
}
pub struct ObjectStorage {
    // Maps: Object Key -> (Vector Index, WeakRef)
    keymap: SlotMap<DefaultKey, (usize, Arc<Py<PyWeakref>>)>,
    
    // The actual objects (Packed array)
    storage: Vec<Object>,
    
    glue_data: Vec<GlueData>,
    
    physics_world: RapierWorld,
}
impl ObjectStorage {
    pub fn new() -> ObjectStorage {
        ObjectStorage {
            keymap: SlotMap::default(),
            storage: Vec::new(),
            glue_data: Vec::new(),
            physics_world: RapierWorld::new(),
        }
    }

    pub fn push(&mut self, obj: Object, weak_ref_handle: Py<PyWeakref>) -> DefaultKey {

        
        let idx = self.storage.len(); // the index of where the obj will be placed eventually.
        let key = self.keymap.insert((idx, Arc::new(weak_ref_handle)));

        // gets the glue data.
        let glue  = GlueData{  reverse_lookup: key, obj_handle: self.physics_world.insert_object(&obj, key)};
        self.glue_data.push(glue);

        self.storage.push(obj);
        
        key
    }

    /// Returns the Oject key through the Sender, before the object has been created.
    pub fn quick_push<F: FnOnce()-> Object>(&mut self,
        sender: SyncSender<DefaultKey>,
        weak_ref_handle: Py<PyWeakref>,
        object_factory: F){

        let idx = self.storage.len();
        let key = self.keymap.insert((idx, Arc::new(weak_ref_handle)));
        let _ = sender.send(key);

        let obj = object_factory();

        let glue  = GlueData{  reverse_lookup: key, obj_handle: self.physics_world.insert_object(&obj, key)};
        self.glue_data.push(glue);

        self.storage.push(obj);
    }

    pub fn remove_object(&mut self, key: DefaultKey) {
    
        let idx_to_remove = match self.keymap.get(key) {
            Some(val) => val.0,
            None => panic!("Key not Found"), 
        };
    
        let last_idx = self.storage.len() - 1; 
    
        self.storage.swap(idx_to_remove, last_idx);
        
        self.glue_data.swap(idx_to_remove, last_idx);
    
        if idx_to_remove != last_idx {
            
            let key_of_moved_obj = self.glue_data[idx_to_remove].reverse_lookup;
    
            if let Some((stored_idx, _)) = self.keymap.get_mut(key_of_moved_obj) {
                *stored_idx = idx_to_remove;
            }
        }

        self.storage.pop();

        

        let obj_data=  self.glue_data.pop().expect("tried to remove obj-data, but array is empty.");
        self.physics_world.remove_object(obj_data.obj_handle);
        
        self.keymap.remove(key);
    
        

        let current_len = self.storage.len();
        let current_cap = self.storage.capacity();
    
        let wasted_capacity = current_cap.saturating_sub(current_len);
        
        
    }

    
    pub fn get_pyref(&self, key: DefaultKey) -> Arc<Py<PyWeakref>> {
        self.keymap.get(key).expect("WeakRef not found for key").1.clone()
    }

    /// changing values here, may result in a de-sync with the mesh.
    /// thats why its unsafe.
    pub unsafe fn get_handle_mut(&mut self, key: DefaultKey) -> &mut ObjectHandle{
        let (vec_idx, _) = self.keymap.get(key).expect("key not known to the map.");
        &mut self.glue_data.get_mut(*vec_idx).expect("missing object data").obj_handle
    }

    pub fn get_handle(&self, key: DefaultKey) -> &ObjectHandle{
        let (vec_idx, _) = self.keymap.get(key).expect("key not known to the map.");
        &self.glue_data.get(*vec_idx).expect("missing object data").obj_handle
    }

    /// changing values here, may result in a de-sync with the collider.
    /// thats why its unsafe.
    pub unsafe fn get_mut(&mut self, key: DefaultKey) -> &mut Object {

        let (vec_idx, _) = self.keymap.get(key).expect("key not known to the map.");
        self.storage.get_mut(*vec_idx).expect("missing object")
        
    }


    pub fn get(&self, key: DefaultKey) -> &Object {
        let (vec_idx, _) = self.keymap.get(key).expect("key not known to the map.");
        self.storage.get(*vec_idx).expect("missing object")
    }

    pub fn len(&self)-> usize{
        self.storage.len()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Object> {
        self.storage.iter_mut()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Object> {
        self.storage.iter()
    }


    pub fn step_physics(&mut self, distance: f32){
        self.physics_world.step(distance);
    }
    pub fn does_collide(&mut self, key: DefaultKey)-> bool{
        let (vec_idx, _) = self.keymap.get(key).expect("key not known to the map.");
        let glue  = self.glue_data.get(*vec_idx).expect("missing object for 'does_collide'");
        self.physics_world.has_collision(*(&glue.obj_handle.collider_handle))
    }

    pub fn collides_with(&mut self, key: DefaultKey)-> Vec<DefaultKey>{
        let (vec_idx, _) = self.keymap.get(key).expect("key not known to the map.");
        let glue  = self.glue_data.get(*vec_idx).expect("missing object for 'collides_with'");
        self.physics_world.get_collided_keys(*(&glue.obj_handle.collider_handle))
    }

    pub fn keys_to_py(&mut self, keys: Vec<DefaultKey>)-> Vec<Arc<Py<PyWeakref>>>{
        let mut v = Vec::new();
        for key in keys{
            let p = self.get_pyref(key);
            v.push(p);
        }
        v
    }

    /// the user-provided function is responsible for changing ALL internal obj values.
    pub fn change_obj_location<T: FnOnce(&mut Object)>(&mut self, location: &mq::Vec3,  key: DefaultKey,obj_recalc: T){

        {
            let glue  = *self.get_handle(key);
            self.physics_world.move_object(&glue.rigid_body_handle, location);
        }
        let obj  = unsafe {self.get_mut(key)};
        obj_recalc(obj);
        
    }
    /// the user-provided function is responsible for changing ALL internal obj values.
    pub fn change_obj_rotation<T: FnOnce(&mut Object)>(&mut self, rotation: &mq::Vec3, key: DefaultKey,obj_recalc: T){

        {
            let glue  = *self.get_handle(key);
            self.physics_world.rotate_object(&glue.rigid_body_handle, rotation);
        }
        let obj  = unsafe {self.get_mut(key)};
        obj_recalc(obj);
        
    }
    /// the user-provided function is responsible for changing ALL internal obj values.
    pub fn change_obj_scale<T: FnOnce(&mut Object)>(&mut self, scale: &mq::Vec3, key: DefaultKey,obj_recalc: T){

        todo!()
        
    }
}
