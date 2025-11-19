use crate::engine::{structures::Rectangle, Objects::Cube::Cube};
use pyo3::prelude::*;
use pyo3::types::PyWeakref;
use pyo3::prelude::PyWeakrefMethods;
use rustc_hash::FxHashMap;
use pyo3::types::{ PyModule};
pub enum Object {
    Rectangle(Rectangle),
    Cube(Cube),
}

pub struct ObjectSortage {
    // Maps: Object Key -> (Vector Index, WeakRef)
    keymap: FxHashMap<usize, (usize, Py<PyWeakref>)>,
    
    // The actual objects
    storage: Vec<Object>,
    
    // NEW: Maps: Vector Index -> Object Key
    // We need this to know WHO we moved during a swap-remove.
    reverse_key_lookup: Vec<usize>,

    // NEW: A counter to ensure unique keys even after removal
    next_id: usize,
}

impl ObjectSortage {
    pub fn new() -> ObjectSortage {
        ObjectSortage {
            keymap: FxHashMap::default(),
            storage: Vec::new(),
            reverse_key_lookup: Vec::new(),
            next_id: 0,
        }
    }

    // New signature accepts the persistent, Send handle
pub fn push(&mut self, obj: Object, obj_handle: Py<PyAny>) -> PyResult<usize> {

    // 1. Generate a unique Key
    let key = self.next_id;
    self.next_id += 1;

    // 2. Store the object and the reverse mapping (Pure Rust operations)
    self.storage.push(obj);
    self.reverse_key_lookup.push(key);

    // 3. 🔑 Acquire the GIL for Python operations 🔑
    pyo3::Python::attach(|py| {
        
        // 4. Bind the persistent handle to the current thread's GIL token
        let py_obj = obj_handle.bind(py); 

        // 5. Create WeakRef (PyO3 operations)
        let weakref_mod = pyo3::types::PyModule::import(py, "weakref")?;
        let weak_bound: Bound<'_, PyWeakref> = weakref_mod
            .call_method1("ref", (py_obj,))?
            .downcast_into::<pyo3::types::PyWeakref>()?;

        // 6. Update KeyMap: Key -> (Index, WeakRef)
        let idx = self.storage.len() - 1;
        self.keymap.insert(key, (idx, weak_bound.unbind()));
        
        // Return key from the closure, handling the PyResult conversion
        Ok(key)
    })
}

    pub fn remove_object(&mut self, key_to_remove: usize) {
        
        let idx_to_remove = match self.keymap.get(&key_to_remove) {
            Some(val) => val.0,
            None => return,
        };

        let last_idx = self.storage.len() - 1;

        // 2. Swap the Object in storage
        self.storage.swap(idx_to_remove, last_idx);
        
        // 3. Swap the Reverse Map (so we know who moved where)
        self.reverse_key_lookup.swap(idx_to_remove, last_idx);

        // 4. Handle the object that got moved (if we didn't just remove the very last item)
        if idx_to_remove != last_idx {
            // The object that WAS at `last_idx` is now at `idx_to_remove`.
            // We need to update its entry in the keymap.
            
            // Look up the Key of the object that got moved using our Reverse Map
            let key_of_moved_obj = self.reverse_key_lookup[idx_to_remove];

            // Update the keymap for that object to point to its new index
            if let Some((stored_idx, _)) = self.keymap.get_mut(&key_of_moved_obj) {
                *stored_idx = idx_to_remove;
            }
        }

        // 5. Remove the old data (now sitting at the end)
        self.storage.pop();
        self.reverse_key_lookup.pop();
        self.keymap.remove(&key_to_remove);
    }

    pub fn get_pyref(&self, py: Python<'_>, key: usize) -> PyResult<Option<Py<PyAny>>> {
        let persistent_weak_ref = match self.keymap.get(&key) {
            Some((_, r)) => r,
            None => return Ok(None),
        };

        let bound_weak_ref = persistent_weak_ref.bind(py);

        match bound_weak_ref.upgrade() {
            Some(bound_strong) => Ok(Some(bound_strong.unbind())),
            None => Ok(None),
        }
    }

    pub fn get_mut(&mut self, key: usize) -> &mut Object {
        let (vec_idx, _) = self.keymap.get(&key).expect("missing key");
        // We access storage directly using the index from the map
        self.storage.get_mut(*vec_idx).expect("missing object")
    }

    pub fn get(&self, key: usize) -> &Object {
        let (vec_idx, _) = self.keymap.get(&key).expect("missing key");
        self.storage.get(*vec_idx).expect("missing object")
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Object> {
        self.storage.iter_mut()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Object> {
        self.storage.iter()
    }
}