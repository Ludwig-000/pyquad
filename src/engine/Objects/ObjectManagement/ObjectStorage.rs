use crate::engine::{structures::Rectangle, Objects::Cube::Cube};
use pyo3::prelude::*;
use pyo3::types::PyWeakref;
use slotmap::*;

pub enum Object {
    Rectangle(Rectangle),
    Cube(Cube),
}

pub struct ObjectStorage {
    // Maps: Object Key -> (Vector Index, WeakRef)
    keymap: SlotMap<DefaultKey, (usize, Py<PyWeakref>)>,
    
    // The actual objects (Packed array)
    storage: Vec<Object>,
    
    // Maps: Vector Index -> Object Key
    // We need this to know WHO we moved during a swap-remove.
    reverse_key_lookup: Vec<DefaultKey>,
}

impl ObjectStorage {
    pub fn new() -> ObjectStorage {
        ObjectStorage {
            keymap: SlotMap::default(),
            storage: Vec::new(),
            reverse_key_lookup: Vec::new(),
        }
    }

    pub fn push(&mut self, obj: Object, weak_ref_handle: Py<PyWeakref>) -> DefaultKey {
        // 1. Store the object in the packed vector
        self.storage.push(obj);
        let idx = self.storage.len() - 1;

        // 2. Insert into KeyMap to generate the unique Key
        // SlotMap handles the generation of the key here.
        let key = self.keymap.insert((idx, weak_ref_handle));

        // 3. Store the reverse mapping
        self.reverse_key_lookup.push(key);
        
        // Return the SlotMap key
        key
    }

    pub fn remove_object(&mut self, key_to_remove: DefaultKey) {
    
        // 1. Check if key exists and get its index in storage
        let idx_to_remove = match self.keymap.get(key_to_remove) {
            Some(val) => val.0,
            None => panic!("Key not Found"), 
        };
    
        let last_idx = self.storage.len() - 1; 
    
        // 2. Swap the Object in storage
        self.storage.swap(idx_to_remove, last_idx);
        
        // 3. Swap the Reverse Map
        self.reverse_key_lookup.swap(idx_to_remove, last_idx);
    
        // 4. Handle the object that got moved 
        if idx_to_remove != last_idx {
            
            let key_of_moved_obj = self.reverse_key_lookup[idx_to_remove];
    
            // Update the keymap for that object to point to its new index
            if let Some((stored_idx, _)) = self.keymap.get_mut(key_of_moved_obj) {
                *stored_idx = idx_to_remove;
            }
        }
    
        // 5. Remove the old data (now sitting at the end)
        self.storage.pop();
        self.reverse_key_lookup.pop();
        
        // Remove from SlotMap
        self.keymap.remove(key_to_remove);
    
        // --- Memory Optimization: Threshold-Based Shrinking ---
        let current_len = self.storage.len();
        let current_cap = self.storage.capacity();
    
        // Calculate the unused capacity
        let wasted_capacity = current_cap.saturating_sub(current_len);
    
        // Shrink only if the vector is large (over 100 elements) AND 
        // the wasted capacity is more than 10% of the total capacity.
        if current_len > 100 && wasted_capacity * 10 > current_cap {
            self.storage.shrink_to_fit(); 
            self.reverse_key_lookup.shrink_to_fit();
        }
    }



    /// This function WILL panic ( lol )
    pub fn get_pyref(&self, key: DefaultKey) -> Py<PyWeakref> {
        
        match self.keymap.get(key) { 
            Some((_, r)) => {
                unsafe {
                r.clone_ref(pyo3::Python::assume_attached())
                }

            },
            None => panic!("WeakRef not found for key"),
        }
    }

    pub fn get_mut(&mut self, key: DefaultKey) -> &mut Object {
        let (vec_idx, _) = self.keymap.get(key).expect("missing key");
        // We access storage directly using the index from the map
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