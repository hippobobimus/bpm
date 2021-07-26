use bevy::prelude::*;

use crate::physics::{Drag, Force, Gravity, Mass, Thrust, Velocity};

/// A component bundle for 'physics' entities.
#[derive(Bundle, Default)]
pub struct PhysicsBundle {
    pub mass: Mass,
    pub velocity: Velocity,
    pub force: Force,
    pub drag: Drag,
    pub gravity: Gravity,
    pub thrust: Thrust,
}
