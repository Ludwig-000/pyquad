use rapier3d::prelude::*;
use glam::{Vec3, Quat};

use rapier3d::na::{Isometry3, Vector3};
pub fn myFun() {


    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    // Gravity is irrelevant for static cubes, but we define it anyway
    let gravity = vector![0.0, -9.81, 0.0];
    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhaseBvh::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();


    let cube_positions: [Vec3; 3] = [
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 2.0),
    ];

    let half_extents = Vec3::new(0.5, 0.5, 0.5);

    for pos in cube_positions.iter() {

        let na_pos = Vector3::new(pos.x, pos.y, pos.z);
        let iso = Isometry3::new(na_pos, Vector3::zeros());


        let rigid_body = RigidBodyBuilder::fixed()
            .position(iso)
            .build();

        let rb_handle = rigid_body_set.insert(rigid_body);

        let collider = ColliderBuilder::cuboid(half_extents.x, half_extents.y, half_extents.z)
            .build();

        collider_set.insert_with_parent(collider, rb_handle, &mut rigid_body_set);
    }

    println!("Inserted {} static cubes into the physics world.", cube_positions.len());


    physics_pipeline.step(
        &gravity,
        &integration_parameters,
        &mut island_manager,
        &mut broad_phase,
        &mut narrow_phase,
        &mut rigid_body_set,
        &mut collider_set,
        &mut impulse_joint_set,
        &mut multibody_joint_set,
        &mut ccd_solver,
        &physics_hooks,
        &event_handler,
    );

    println!("Physics step completed.");
}