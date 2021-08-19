use bevy::math::DVec3;

use crate::{
    physics::components::PhysTransform,
    physics::shapes::Collidable,
};

/// An infinite plane in a 3D coordinate system, described by the plane normal vector. Its
/// position, described by any point on the plane, is not stored directly.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Plane {
    normal: DVec3,
}

impl Plane {
    /// Creates a new infinite plane with the given vector normal.
    pub fn new(normal: DVec3) -> Self {
        Self { normal: normal.normalize() }
    }

    /// Returns a reference to the vector normal of the plane.
    pub fn normal(&self) -> &DVec3 {
        &self.normal
    }
}

impl Collidable for Plane {
    /// Calculates and returns the closest point on the plane, located by the given point on the
    /// plane, to the given target point.
    fn closest_point_to(&self, transform: &PhysTransform, target: DVec3) -> DVec3 {
        // Use plane equation n.(X - P) = 0 where P is the location of the plane and X is any point
        // on the plane with R = Q - tn where Q is the target and R is the closest point on the
        // plane. Substitute R for X.

        // Convert target into local body coords.
        let target_local = transform.get_point_in_local_space(target);

        // In local coords the origin is on the plane so (X - P) becomes X.
        let t = self.normal.dot(target);  // assume unit normal so n.n = 1.

        let result_local = target_local - t * self.normal;

        // Convert the result back into global coords.
        transform.get_point_in_global_space(result_local)
    }

    /// Calculates and returns the shortest distance between the Plane, with the given transform,
    /// and the target point in global coords.
    fn shortest_distance_to(&self, transform: &PhysTransform, target: DVec3) -> f64 {
        let target_local = transform.get_point_in_local_space(target);
        self.normal.dot(target_local)
    }
}
