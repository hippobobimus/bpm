use bevy::{
    prelude::*,
    math::DVec3,
};

use crate::{
    constants,
    physics::components::{
        AngularVelocity,
        BoundaryCollider,
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

/// A component bundle that adds rigid-body physics to an entity. Supports cuboids and spheres.
#[derive(Bundle)]
pub struct PhysicsColliderBundle {
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

impl PhysicsColliderBundle {
    /// Creates a new PhysicsColliderBundle for a cuboid body with the given mass, transform and extents.
    pub fn cuboid(mass: f64, extents: DVec3, transform: PhysTransform) -> Self {
        Self {
            collider: Collider::new(Cuboid::new(extents)),
            inertia_tensor: InertiaTensor::cuboid(mass, extents.x, extents.y, extents.z),
            mass: Mass::new(mass),
            transform,
            ..Default::default()
        }
    }

    /// Creates a new PhysicsColliderBundle for a spherical body with the given mass, transform and extents.
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

impl Default for PhysicsColliderBundle {
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

/// A component bundle that adds rigid-body physics to an entity. Supports boundary planes as
/// half-spaces.
#[derive(Bundle)]
pub struct PhysicsBoundaryBundle {
    pub boundary_collider: BoundaryCollider,
    pub mass: Mass,
    pub transform: PhysTransform,
}

impl PhysicsBoundaryBundle {
    /// Creates a new PhysicsBoundaryBundle for a half-space in the x-z plane, intersecting the origin and with a
    /// normal in the y-axis, subsequently transformed by the given PhysTransform.
    pub fn new(transform: PhysTransform) -> Self {
        Self {
            boundary_collider: BoundaryCollider::new(&transform),
            mass: Mass::from_inverse(0.0), // infinite mass, i.e. cannot move.
            transform,
        }
    }
}

impl Default for PhysicsBoundaryBundle {
    fn default() -> Self {
        Self {
            boundary_collider: BoundaryCollider::new(&PhysTransform::default()),
            mass: Mass::from_inverse(0.0), // infinite mass, i.e. cannot move.
            transform: Default::default(),
        }
    }
}
