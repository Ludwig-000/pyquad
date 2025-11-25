use crate::engine::Objects::{Cube::Cube, ObjectManagement::ObjectStorage::*};


use macroquad::miniquad::{BufferType, BufferUsage, BufferSource};


pub fn draw_all_Objects(obj: &ObjectStorage){
    
    let it = obj.iter();
    
    it.map(|item|{
        match item{
            Object::Cube(cube)=> cube.draw(),
            _ => todo!(),
        }
    }).collect()
    
}