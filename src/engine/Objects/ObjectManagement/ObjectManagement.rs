use crate::engine::Objects::{Cube::Cube, ObjectManagement::ObjectStorage::*};





pub fn draw_all_Objects(obj: &ObjectSortage){
    let it = obj.iter();
    

    it.map(|item|{
        match item{
            Object::Cube(cube)=> cube.draw(),
            _ => todo!(),
        }
    }).collect()
    
}