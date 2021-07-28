pub mod components;
pub mod entity;
mod oct_tree;
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
}

use bevy::prelude::*;

use systems::{forces, integrator};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(
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
                    .after("forces")
            );
    }
}
