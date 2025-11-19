use lazy_static::lazy_static;
use rapier3d::prelude::*;
use glam::Vec3;
use std::sync::Mutex;


lazy_static! {
    static ref PHYSICS_WORLD: Mutex<all_my_friends> = Mutex::new({
        let collider_set = ColliderSet::new();
        let broad_phase = BroadPhaseBvh::new();
        let narrow_phase = NarrowPhase::new();
        let rigid_body_set = RigidBodySet::new();

        all_my_friends::new(broad_phase, collider_set, narrow_phase, rigid_body_set)
    });
}

pub fn physics_thread(){
    // todo
}

pub fn get_physics_world() -> impl std::ops::DerefMut<Target = all_my_friends> + 'static {
    PHYSICS_WORLD.lock().unwrap()
}


// This struct holds the handles to a specific object in the physics world
pub struct Cube {
    pub rigid_body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
}
pub struct all_my_friends{
    bvh: BroadPhaseBvh,
    coll: ColliderSet,
    narrowP: NarrowPhase,
    rigidBS: RigidBodySet,
}
impl all_my_friends{
    pub fn new(bvh: BroadPhaseBvh,coll: ColliderSet, narrowP: NarrowPhase, rigidBS: RigidBodySet)-> all_my_friends{
        all_my_friends{bvh,coll,narrowP,rigidBS}
    }
    pub fn insert_cube(&mut self) -> Cube {
        // 1. Create a dynamic rigid-body.
        let rigid_body = RigidBodyBuilder::dynamic()
            // We use the `vector!` macro from the rapier prelude
            .translation(vector![0.0, 10.0, 0.0]) 
            .build();
        
        // 2. Create a cuboid collider.
        let collider = ColliderBuilder::cuboid(0.5, 0.5, 0.5)
            .restitution(0.7) // Make it a bit bouncy
            .density(1.0)     // Give it a default density
            .build();
        
        // 3. Insert the rigid-body into the set.
        let rigid_body_handle = self.rigidBS.insert(rigid_body);
        
        // 4. Insert the collider, attaching it to the rigid-body.
        let collider_handle = self.coll.insert_with_parent(
            collider,
            rigid_body_handle,
            &mut self.rigidBS
        );

        // 5. Return the handles wrapped in our Cube struct
        Cube { rigid_body_handle, collider_handle }
    }

    /**
     * Checks if the given Cube is currently colliding with anything.
     *
     * This function must be called *after* a physics step has been computed
     * (e.g., after `PhysicsPipeline::step()`).
     *
     * It checks for both:
     * 1. Physical contacts (hard collisions).
     * 2. Sensor intersections (overlaps).
     *
     * Returns `true` if any contact or intersection is found, `false` otherwise.
     */
    pub fn check_for_collision(&mut self, cube: Cube) -> bool {
        // We can use `&self.narrowP` even though the method is `&mut self`.
        // Rust will automatically "downgrade" the `&mut self` to an `&self`.
        
        // Check for any physical contacts
        let has_contacts = self.narrowP
            .contact_pairs_with(cube.collider_handle)
            .next() // Get the first contact, if any
            .is_some(); // Check if an item existed
        
        // Check for any sensor intersections
        let has_intersections = self.narrowP
            .intersection_pairs_with(cube.collider_handle)
            .next() // Get the first intersection, if any
            .is_some(); // Check if an item existed
        
        // Return true if it's either contacting or intersecting
        has_contacts || has_intersections
    }
}




fn init_rapier()->  all_my_friends {

    let collider_set = ColliderSet::new();
    let broad_phase = BroadPhaseBvh::new();
    let narrow_phase = NarrowPhase::new();
    let rigid_body_set = RigidBodySet::new();

    all_my_friends::new(broad_phase, collider_set, narrow_phase, rigid_body_set)
}
































// // unsafe version
// static mut PHYSICS_WORLD: Option<all_my_friends> = None;

// pub fn initialize_physics() {
//     unsafe { 
//         match PHYSICS_WORLD {
//             Some(_) => panic!("Rapier can only be initialized once."),
//             None => PHYSICS_WORLD = Some( init_rapier()  ),
//         }
//      }
// }


// #[allow(static_mut_refs)]
// pub fn get_physics_world() -> &'static mut all_my_friends {
//     unsafe {
//         let world_ref: &mut all_my_friends = PHYSICS_WORLD
//             .as_mut()
//             .expect("Physics world not initialized!");

//         let ptr: *mut all_my_friends = world_ref as *mut _;
//         &mut *ptr
//     }
// }