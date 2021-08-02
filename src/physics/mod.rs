pub mod components;
pub mod entity;
mod oct_tree;
mod shapes;
pub mod systems;

// Re-exports
pub use components::{
    AngularVelocity,
    Drag,
    Force,
    Gravity,
    InertiaTensor,
    Mass,
    Rotator,
    Thrust,
    Torque,
    Velocity,
};
pub use entity::PhysicsBundle;

pub mod prelude {
    #[doc(hidden)]
    pub use super::components::{
        AngularVelocity,
        Drag,
        Force,
        Gravity,
        InertiaTensor,
        Mass,
        PhysTransform,
        Rotator,
        Thrust,
        Torque,
        Velocity,
    };
    pub use super::entity::PhysicsBundle;
    pub use super::oct_tree::OctTree;
    pub use super::shapes::{
        Aabb3D,
        Plane,
        Sphere,
    };
}

use bevy::prelude::*;

use systems::{collision_detection, forces, integrator};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(
                collision_detection::build_tree.system()
            )
            .add_system(
                forces::reset_force_and_torque_accumulators.system()
                    .label("reset")
            )
            .add_system(
                forces::force_accumulation.system()
                    .label("forces")
                    .after("reset")
            )
            .add_system(
                integrator::integrator.system()
                    .label("integrator")
                    .after("forces")
            )
            .add_system_set(
                SystemSet::new()
                    .label("collision detection")
                    .after("integrator")
                    .with_system(collision_detection::update_tree.system()
                                 .label("tree update")
                    )
                    .with_system(collision_detection::detect_collisions.system()
                                 .after("tree update")
                    )
            );
    }
}
