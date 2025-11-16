use std::{fmt::format, hash::Hash};

use crate::engine::{Objects::Cube::Cube, structures::Rectangle};
use pyo3_stub_gen::inventory::iter;
use rustc_hash::{FxBuildHasher, FxHashMap, FxHasher};


pub enum Object {
    Rectangle(Rectangle),
    Cube(Cube),
}

pub struct ObjectSortage{
    keymap: FxHashMap<usize, usize>,
    storage: Vec<Object>,
}


impl ObjectSortage{
    pub fn new()-> ObjectSortage{
        ObjectSortage{
            keymap: FxHashMap::default(),
            storage: Vec::new(),
        }
    }
    pub fn push(&mut self, obj: Object)-> usize{


        let key = self.storage.len();
        self.storage.push( obj );
        self.keymap.insert( key , key);

        key
    }
    pub fn remove_object(&mut self, object_key: usize) {
        let idx = *self.keymap
            .get(&object_key)
            .expect(&format!("Missing key in object storage: {object_key}"));
    
        let last_idx = self.storage.len() - 1;
    
        self.storage.swap(idx, last_idx);
    
        self.storage.pop();
    
        if idx != last_idx {
            let moved_key = last_idx;
            self.keymap.insert(moved_key, idx);
        }
    
        self.keymap.remove(&object_key);
    }

    pub fn get_mut(&mut self, key: usize)-> &mut Object{
        let vec_key= self.keymap.get(&key  ).expect("missing key");
        let obj  =  self.storage.get_mut(*vec_key).expect("missing object");
        obj
    }
    pub fn get(&self, key: usize)-> &Object{
        let vec_key= self.keymap.get(&key  ).expect("missing key");
        let obj  =  self.storage.get(*vec_key).expect("missing object");
        obj
    }
}




impl<'a> IntoIterator for &'a ObjectSortage {
    type Item = &'a Object;
    type IntoIter = std::slice::Iter<'a, Object>;

    fn into_iter(self) -> Self::IntoIter {
        self.storage.iter()
    }
}

impl<'a> IntoIterator for &'a mut ObjectSortage {
    type Item = &'a mut Object;
    type IntoIter = std::slice::IterMut<'a, Object>;

    fn into_iter(self) -> Self::IntoIter {
        self.storage.iter_mut()
    }
}