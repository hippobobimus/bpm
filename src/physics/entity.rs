use bevy::{
    prelude::*,
    math::DVec3,
};

use crate::{
    constants,
    physics::components::{
        AngularVelocity,
        Collider,
        Drag,
        Force,
        Gravity,
        InertiaTensor,
        Mass,
        Thrust,
        Torque,
        PhysTransform,
        Velocity,
    },
    physics::shapes::{
        Cuboid,
        Sphere,
    },
};

/// A component bundle that adds rigid-body physics to an entity.
#[derive(Bundle)]
pub struct PhysicsBundle {
    pub angular_velocity: AngularVelocity,
    pub collider: Collider,
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

impl PhysicsBundle {
    /// Creates a new PhysicsBundle for a cuboid body with the given mass, transform and extents.
    pub fn cuboid(mass: f64, extents: DVec3, transform: PhysTransform) -> Self {
        Self {
            collider: Collider::new(Cuboid::new(extents)),
            inertia_tensor: InertiaTensor::cuboid(mass, extents.x, extents.y, extents.z),
            mass: Mass::new(mass),
            transform,
            ..Default::default()
        }
    }

    /// Creates a new PhysicsBundle for a spherical body with the given mass, transform and extents.
    pub fn sphere(mass: f64, radius: f64, transform: PhysTransform) -> Self {
        Self {
            collider: Collider::new(Sphere::new(radius)),
            inertia_tensor: InertiaTensor::sphere(mass, radius),
            mass: Mass::new(mass),
            transform,
            ..Default::default()
        }
    }
}

impl Default for PhysicsBundle {
    fn default() -> Self {
        Self {
            angular_velocity: Default::default(),
            collider: Collider::new(Sphere::new(constants::DEFAULT_RADIUS)),
            drag: Default::default(),
            force: Default::default(),
            gravity: Default::default(),
            inertia_tensor: InertiaTensor::sphere(constants::DEFAULT_MASS, constants::DEFAULT_RADIUS),
            mass: Mass::new(1.0),
            thrust: Default::default(),
            torque: Default::default(),
            transform: Default::default(),
            velocity: Default::default(),
        }
    }
}
