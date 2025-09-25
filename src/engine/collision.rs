use rapier3d::geometry::{ ColliderBuilder, ColliderHandle};
use rapier3d::prelude::*;




//fn collision() {
//    // Broad-phase acceleration structure (DBVT)
//    let mut broad_phase = DefaultBroadPhase::new();
//    let mut colliders = ColliderSet::new();

//    // Insert a few cubes
//    let cube1 = colliders.insert(
//        ColliderBuilder::cuboid(1.0, 1.0, 1.0).translation(vector![0.0, 0.0, 0.0])
//    );
//    let cube2 = colliders.insert(
//        ColliderBuilder::cuboid(1.0, 1.0, 1.0).translation(vector![2.0, 0.0, 0.0])
//    );
//    let cube3 = colliders.insert(
//        ColliderBuilder::cuboid(1.0, 1.0, 1.0).translation(vector![0.5, 0.0, 0.0])
//    );


//    for (handle, collider) in colliders.iter() {
//        broad_phase.create_proxy(handle, collider.shape().compute_aabb(&collider.position()), 0);
//    }

//    broad_phase.find_pairs(|h1: ColliderHandle, h2: ColliderHandle| {
//        println!("Potential collision between {:?} and {:?}", h1, h2);
//    });
//}