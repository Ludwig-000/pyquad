use lazy_static::lazy_static;
use rapier3d::{pipeline, prelude::*};
use glam::Vec3;
use std::sync::Mutex;
use slotmap::{DefaultKey, Key, KeyData};
use crate::engine::Objects::ObjectManagement::ObjectStorage as obj;
// This struct holds the handles to a specific object in the physics world

pub fn physics_thread(){
    let c = RapierWorld::new();


}
pub struct ObjectHandle {
    pub rigid_body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
}

pub fn manually_move_object(handle: &RigidBodyHandle, pos: &Vec3, rbs: &mut RigidBodySet) {
    if let Some(rb) = rbs.get_mut(*handle) {
        rb.set_translation(vector![pos.x, pos.y, pos.z], true);
    }
}

fn extract_object_transforms(obj: &obj::Object)-> Transforms{
    match obj{
        obj::Object::Cube(cube) => Transforms { pos: &cube.position, rot: &cube.rotation, scale: &cube.scale },
        _ => todo!()
    }
}
struct Transforms<'a>{
    pos: &'a Vec3,
    rot: &'a Vec3,
    scale: &'a Vec3,
}
pub struct RapierWorld{
    pipeline: CollisionPipeline,
    bvh: BroadPhaseBvh,
    coll: ColliderSet,
    narrowP: NarrowPhase,
    rigidBS: RigidBodySet,
}
impl RapierWorld{

    pub fn new()-> RapierWorld{
        RapierWorld{
            pipeline: CollisionPipeline::new(),
            bvh: BroadPhaseBvh::new(),
            coll: ColliderSet::new(),
            narrowP: NarrowPhase::new(),
            rigidBS: RigidBodySet::new(),
        }
    }


    pub fn step(&mut self, distance: f32) {
        self.pipeline.step(
            distance,
            &mut self.bvh,
            &mut self.narrowP,
            &mut self.rigidBS,
            &mut self.coll,
            &(),
            &(),
        );
    }
    pub fn insert_object(&mut self, obj: &obj::Object, key: DefaultKey) -> ObjectHandle {
        let t: Transforms<'_>  = extract_object_transforms(obj);
        
        let rigid_body = RigidBodyBuilder::kinematic_position_based()
            .translation(vector![t.pos.x,t.pos.y,t.pos.z])
            .rotation(vector![t.rot.x,t.rot.y,t.rot.z])
            .build();
        
        // NOTE: cuboid takes half-extents. We divide size by 2.0.
        let mut collider  = match obj{
            obj::Object::Cube(_) => {
                ColliderBuilder::cuboid(t.scale.x / 2.0, t.scale.y / 2.0, t.scale.z / 2.0)
                    .restitution(0.7)
                    .density(1.0)
                    .build()
            },
            _ => todo!(),
        };
        
        collider.user_data = key_to_u128(key);

        let rigid_body_handle = self.rigidBS.insert(rigid_body);
        
        
        let collider_handle = self.coll.insert_with_parent(
            collider,
            rigid_body_handle,
            &mut self.rigidBS
        );
    
        ObjectHandle { rigid_body_handle, collider_handle }
    }

    pub fn check_for_collision(&mut self, collider_handle: ColliderHandle) -> bool {
        
        let has_contacts = self.narrowP
            .contact_pairs_with(collider_handle)
            .next() 
            .is_some();
        
        let has_intersections = self.narrowP
            .intersection_pairs_with(collider_handle)
            .next()
            .is_some();
        
        has_contacts || has_intersections
    }


    pub fn get_collided_keys(&self, handle: ColliderHandle) -> Vec<DefaultKey> {
    
        let pairs =  self.narrowP.contact_pairs_with(handle).filter_map(|collider|{
            if collider.has_any_active_contact{
                let other_handle = if collider.collider1 == handle { collider.collider2 } else { collider.collider1 };
                
                let h = self.coll.get(other_handle).expect("Collider handle was found, but no matching collider.");
                Some (u128_to_key(h.user_data))
            }
            else{ None }
        });

        let pairs2 =  self.narrowP.intersection_pairs_with(handle).filter_map(|(h1,h2,intersecting)|{
            if intersecting{
                let other_handle = if h1 == handle { h2 } else { h1 };

                let h = self.coll.get(other_handle).expect("Collider handle was found, but no matching collider.");
                Some (u128_to_key(h.user_data))
            }
            else{ None }
        });

        pairs.chain(pairs2).collect()

    }
    
    
}



// Helper to turn that u128 back into your SlotMap Key
fn u128_to_key(val: u128) -> DefaultKey {
    let as_u64 = val as u64;
    KeyData::from_ffi(as_u64).into()
}
// Convert SlotMap Key -> u128 for Rapier
fn key_to_u128(key: DefaultKey) -> u128 {
    let data = key.data();
    let as_u64 = data.as_ffi(); // This gives you the internal bit representation
    as_u64 as u128
}
