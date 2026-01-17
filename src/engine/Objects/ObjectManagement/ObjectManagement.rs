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
                _ => todo!(),
            }
        }).collect();

        
        // let  unitCube: CubeMesh = UnitCube::create_unit_cube();
        // for object in obj.iter(){
        //     match object{
        //         Object::Cube(cube)=> {
        //             draw_from_unit(&unitCube,cube, gl);
        //         },
        //         _ => todo!(),
        //     }
        // }
    }

    // unsafe {
    //     let gl: &mut macroquad::prelude::QuadGl = macroquad::prelude::get_internal_gl().quad_gl;
        
    //     // Setup state once
    //     gl.draw_mode(mq::DrawMode::Triangles);
    //     gl.texture(None);

    //     const unit_cube: CubeMesh = UnitCube::create_unit_cube();
        
        
    //     let mut scratch_buffer: Vec<mq::Vertex> = Vec::with_capacity(unit_cube.vertices.len());

    //     for object in obj.iter() {
    //         match object {
    //             Object::Cube(cube) => {
    //                 // We pass the scratch buffer to be filled
    //                 draw_from_unit(&unit_cube, cube, gl, &mut scratch_buffer);
    //             },
    //             _ => todo!(),
    //         }
    //     }
    // }

    

    
}

// use miniquad::*;

// pub fn draw_from_unit(
//     unit_cube: &CubeMesh, 
//     cube: &Cube, 
//     gl: &mut macroquad::prelude::QuadGl,
//     scratch_buffer: &mut Vec<mq::Vertex>
// ) {

//     let mat4 = get_model_matrix(cube.position, cube.rotation, cube.scale);

//     scratch_buffer.clear();

    
//     for v in &unit_cube.vertices {
//         let mut new_v = *v;
        
//         let pos_glam = Vec3::new(v.position.x, v.position.y, v.position.z);
        
//         let transformed_pos = mat4.transform_point3(pos_glam);
        
//         new_v.position = mq::Vec3::new(transformed_pos.x, transformed_pos.y, transformed_pos.z);
        
//         scratch_buffer.push(new_v);
//     }

    
//     gl.geometry(scratch_buffer, &unit_cube.indices);
// }

// pub fn get_model_matrix(pos: mq::Vec3, rot: mq::Vec3, scale: mq::Vec3) -> Mat4 {
//     let rotation_quat = Quat::from_euler(glam::EulerRot::XYZ, rot.x, rot.y, rot.z);
    
//     let g_scale = Vec3::new(scale.x, scale.y, scale.z);
//     let g_pos = Vec3::new(pos.x, pos.y, pos.z);
    
//     Mat4::from_scale_rotation_translation(g_scale, rotation_quat, g_pos)
// }