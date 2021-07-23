use bevy::prelude::*;

use crate::{
    physics::{forces, integrator},
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(forces::force_accumulation.system())
            .add_system(integrator::integrator.system());
    }
}
