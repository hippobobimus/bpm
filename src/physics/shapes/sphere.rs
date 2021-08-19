use bevy::math::DVec3;

use crate::{
    physics::components::{
        PhysTransform,
    },
    physics::shapes::{
        Collidable,
        CollisionPrimative,
    },
};

/// A sphere described by its radius only. The centre position is not stored directly.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    radius: f64,
}

impl Sphere {
    /// Creates a new Sphere with the given radius.
    pub fn new(radius: f64) -> Self {
        Self {
            radius,
        }
    }

    /// Returns the radius of the Sphere.
    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl CollisionPrimative for Sphere {
    /// The bounding sphere is the Sphere itself.
    fn bounding_sphere(&self) -> &Sphere {
        self
    }
}

impl Collidable for Sphere {
    /// Calculates and returns the closest point on the Sphere centred at the given position to the
    /// given target point. The calculation is made by taking the normalised vector between the
    /// circle's centre and the target point and scaling it by the circle's radius.
    fn closest_point_to(&self, transform: &PhysTransform, target: DVec3) -> DVec3 {
        // Perform calculation in global coords. Sphere centre is the transform translation.
        let centre = transform.translation();

        (target - centre).normalize() * self.radius()
    }

    /// Calculates and returns the shortest distance between the Sphere, with the given transform,
    /// and the target point. Will be negative if the target point is inside the Sphere.
    fn shortest_distance_to(&self, transform: &PhysTransform, target: DVec3) -> f64 {
        let centre = transform.translation();

        (target - centre).length() - self.radius()
    }
}
