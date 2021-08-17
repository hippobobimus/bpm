pub mod components;
mod entity;
mod oct_tree;
pub mod shapes;
mod systems;

// Re-exports
pub use entity::PhysicsBundle;

/// 'use physics::prelude::*;' to import common components, bundles and plugins.
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
    pub use super::entity::PhysicsBundle;
    pub use super::shapes::{
        CollisionPrimative,
        Cuboid,
        Plane,
        Sphere,
    };
}

use bevy::prelude::*;

use systems::{collision_detection, forces, integrator};

/// A Bevy plugin that adds systems to support rigid-body physics, including; force handling,
/// integration, collision detection and collision resolution (TBD).
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(
                collision_detection::initialize.system()
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
                    .with_system(collision_detection::broad_phase.system()
                                 .label("broad phase")
                                 .after("tree update")
                    )
                    .with_system(collision_detection::contact_generation.system()
                                 .after("broad phase")
                    )
            );
    }
}
