use crate::engine::Objects::ObjectManagement::{ObjectStorage::*, UnitCube };
use macroquad::prelude::*;
use macroquad::prelude as mq;
use crate::engine::Objects::Cube::*;


pub fn draw_all_Objects(obj: &ObjectStorage, viewMat: macroquad::prelude::Mat4){

    unsafe {

        let gl: &mut macroquad::prelude::QuadGl  = macroquad::prelude::get_internal_gl().quad_gl;
        gl.draw_mode(mq::DrawMode::Triangles);


        let _: () = obj.iter().map(|item|{
            match item{
                Object::Cube(cube)=> cube.draw(gl),
                Object::Mesh(mesh)=> mesh.draw(gl),
                Object::Sphere(sphere)=> sphere.draw(gl),
            }
        }).collect();

    }

}