use bevy::math::DVec3;

use crate::physics::shapes::Collidable;

/// An infinite plane in a 3D coordinate system, described by the plane normal vector. Its
/// position, described by any point on the plane, is not stored directly.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Plane {
    normal: DVec3,
}

impl Plane {
    /// Creates a new infinite plane with the given vector normal.
    pub fn new(normal: DVec3) -> Self {
        Self { normal }
    }

    /// Returns a reference to the vector normal of the plane.
    pub fn normal(&self) -> &DVec3 {
        &self.normal
    }
}

impl Collidable for Plane {
    /// Calculates and returns the closest point on the plane, located by the given point on the
    /// plane, to the given target point.
    fn closest_point_to(&self, plane_position: DVec3, target: DVec3) -> DVec3 {
        // Use plane equation n.(X - P) = 0 where P is the location of the plane and X is any point
        // on the plane with R = Q - tn where Q is the target and R is the closest point on the
        // plane. Substitute R for X.
        let t = self.normal.dot(target - plane_position);  // assume unit normal so n.n = 1.

        target - t * self.normal
    }

    /// Calculates and returns the shortest distance between the plane, located by the given
    /// position, and the target point.
    fn shortest_distance_to(&self, plane_position: DVec3, target: DVec3) -> f64 {
        self.normal.dot(target - plane_position)
    }
}
