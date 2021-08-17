use bevy::math::DVec3;

use crate::physics::shapes::{
    Collidable,
    CollisionPrimative,
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
    fn closest_point_to(&self, centre: DVec3, target: DVec3) -> DVec3 {
        (target - centre).normalize() * self.radius()
    }

    /// Calculates and returns the shortest distance between the circle, centred at the given
    /// position, and the target point.
    fn shortest_distance_to(&self, centre: DVec3, target: DVec3) -> f64 {
        (target - centre).length() - self.radius()
    }
}
