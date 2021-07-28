use bevy::prelude::*;

use crate::physics::prelude::*;

/// A component bundle for 'physics' entities.
#[derive(Bundle, Default)]
pub struct PhysicsBundle {
    pub angular_velocity: AngularVelocity,
    pub drag: Drag,
    pub force: Force,
    pub gravity: Gravity,
    pub inertia_tensor: InertiaTensor,
    pub mass: Mass,
    pub thrust: Thrust,
    pub torque: Torque,
    pub transform: PhysTransform,
    pub velocity: Velocity,
}
