use bevy::{
    prelude::*,
    math::DVec3,
};

use crate::physics::prelude::*;

/// A component bundle for 'physics' entities.
#[derive(Bundle)]
pub struct PhysicsBundle {
    pub angular_velocity: AngularVelocity,
    pub collision_primative: CollisionPrimative,
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

impl Default for PhysicsBundle {
    fn default() -> Self {
        let mass = 1.0;
        let radius = 1.0;

        Self {
            angular_velocity: Default::default(),
            collision_primative: CollisionPrimative::new(Sphere::new(radius)),
            drag: Default::default(),
            force: Default::default(),
            gravity: Default::default(),
            inertia_tensor: InertiaTensor::sphere(mass, radius),
            mass: Mass::new(1.0),
            thrust: Default::default(),
            torque: Default::default(),
            transform: Default::default(),
            velocity: Default::default(),
        }
    }
}

impl PhysicsBundle {
    pub fn cuboid(mass: f64, extents: DVec3, transform: PhysTransform) -> Self {
        Self {
            collision_primative: CollisionPrimative::new(Cuboid::new(extents)),
            inertia_tensor: InertiaTensor::cuboid(mass, extents.x, extents.y, extents.z),
            mass: Mass::new(mass),
            transform,
            ..Default::default()
        }
    }

    pub fn sphere(mass: f64, radius: f64, transform: PhysTransform) -> Self {
        Self {
            collision_primative: CollisionPrimative::new(Sphere::new(radius)),
            inertia_tensor: InertiaTensor::sphere(mass, radius),
            mass: Mass::new(mass),
            transform,
            ..Default::default()
        }
    }
}
