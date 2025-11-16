use crate::engine::{Objects::Cube::Cube, structures::Rectangle};
use rustc_hash::FxHashMap;


pub enum Object {
    Rectangle(Rectangle),
    Cube(Cube),
}

pub struct ObjectHashmap(FxHashMap<u32, u32>);
impl ObjectHashmap{
    pub fn push_new_Cube(){
        todo!()
    }
}



pub struct AllObectsAndTheirShaders{
    Objects:  Vec<Object>,
}
impl AllObectsAndTheirShaders{
    pub fn new()-> AllObectsAndTheirShaders{
        todo!()
    }
}