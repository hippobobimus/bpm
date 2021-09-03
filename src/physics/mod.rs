pub mod components;
mod entity;
mod oct_tree;
pub mod shapes;
mod systems;

// Re-exports
pub use entity::PhysicsColliderBundle;

/// 'use physics::prelude::*;' to import common components, shapes, bundles and plugins.
pub mod prelude {
    #[doc(hidden)]
    pub use super::components::{
        AngularVelocity,
        BoundaryCollider,
        Collider,
        Drag,
        Gravity,
        InertiaTensor,
        Mass,
        PhysTransform,
        Rotator,
        Thrust,
        Velocity,
    };
    pub use super::entity::{
        PhysicsBoundaryBundle,
        PhysicsColliderBundle,
    };
    pub use super::shapes::{
        CollisionPrimative,
        Cuboid,
        Plane,
        Sphere,
    };
    pub use super::PhysicsPlugin;
}

use bevy::prelude::*;

use systems::{
    collision_detection,
    collision_response,
    force_and_torque,
    integrator,
};

/// A Bevy plugin that adds systems to support rigid-body physics, including; force handling,
/// integration, collision detection and collision resolution (TBD).
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(
                collision_detection::initialize.system()
            )
            .add_system_set(
                force_and_torque::get_system_set()
                    .label("forces")
            )
            .add_system_set(
                integrator::get_system_set()
                    .label("integrator")
                    .after("forces")
            )
            .add_system_set(
                collision_detection::get_system_set()
                    .label("collision detection")
                    .after("integrator")
            )
            .add_system_set(
                collision_response::get_system_set()
                    .label("collision response")
                    .after("collision detection")
            );
    }
}
