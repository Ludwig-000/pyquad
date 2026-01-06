use crate::engine::{structures::Rectangle, Objects::Cube::Cube};
use pyo3::prelude::*;
use pyo3::types::PyWeakref;
use slotmap::*;
use std::sync::Arc;
use crate::engine::collision::Rapier::RapierWorld;
pub enum Object {
    Rectangle(Rectangle),
    Cube(Cube),
}

pub struct ObjectStorage {
    // Maps: Object Key -> (Vector Index, WeakRef)
    keymap: SlotMap<DefaultKey, (usize, Arc<Py<PyWeakref>>)>,
    
    // The actual objects (Packed array)
    storage: Vec<Object>,
    
    // Maps: Vector Index -> Object Key
    reverse_key_lookup: Vec<DefaultKey>,

    // Rapier colission and physics world.
    rapier_colission: RapierWorld,
}

impl ObjectStorage {
    pub fn new() -> ObjectStorage {
        ObjectStorage {
            keymap: SlotMap::default(),
            storage: Vec::new(),
            reverse_key_lookup: Vec::new(),
            rapier_colission: RapierWorld::new(),
        }
    }

    pub fn push(&mut self, obj: Object, weak_ref_handle: Py<PyWeakref>) -> DefaultKey {
        self.storage.push(obj);
        let idx = self.storage.len() - 1;

        let key = self.keymap.insert((idx, Arc::new(weak_ref_handle)));

        self.reverse_key_lookup.push(key);
        
        key
    }

    pub fn remove_object(&mut self, key_to_remove: DefaultKey) {
    
        let idx_to_remove = match self.keymap.get(key_to_remove) {
            Some(val) => val.0,
            None => panic!("Key not Found"), 
        };
    
        let last_idx = self.storage.len() - 1; 
    
        self.storage.swap(idx_to_remove, last_idx);
        
        self.reverse_key_lookup.swap(idx_to_remove, last_idx);
    
        if idx_to_remove != last_idx {
            
            let key_of_moved_obj = self.reverse_key_lookup[idx_to_remove];
    
            if let Some((stored_idx, _)) = self.keymap.get_mut(key_of_moved_obj) {
                *stored_idx = idx_to_remove;
            }
        }
    
        self.storage.pop();
        self.reverse_key_lookup.pop();
        
        self.keymap.remove(key_to_remove);
    
        let current_len = self.storage.len();
        let current_cap = self.storage.capacity();
    
        let wasted_capacity = current_cap.saturating_sub(current_len);
        
        
    }

    
    pub fn get_pyref(&self, key: DefaultKey) -> Arc<Py<PyWeakref>> {
        self.keymap.get(key).expect("WeakRef not found for key").1.clone()
    }

    pub fn get_mut(&mut self, key: DefaultKey) -> &mut Object {

        let (vec_idx, _) = self.keymap.get(key).expect("missing key");
        self.storage.get_mut(*vec_idx).expect("missing object")
        
    }

    pub fn get(&self, key: DefaultKey) -> &Object {
        let (vec_idx, _) = self.keymap.get(key).expect("missing key");
        self.storage.get(*vec_idx).expect("missing object")
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Object> {
        self.storage.iter_mut()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Object> {
        self.storage.iter()
    }
}
