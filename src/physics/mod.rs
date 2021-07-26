pub mod components;
pub mod entity;
pub mod systems;

// Re-exports
pub use components::{Drag, Force, Gravity, Mass, Thrust, Velocity};
pub use entity::PhysicsBundle;

use bevy::prelude::*;

use systems::{forces, integrator};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(forces::force_accumulation.system())
            .add_system(integrator::integrator.system());
    }
}
