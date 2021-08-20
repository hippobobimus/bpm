use bevy::math::DVec3;

use crate::{
    physics::components::PhysTransform,
    physics::shapes::Collidable,
};

/// An infinite fixed plane in a 3D coordinate system, described by the plane normal vector. Its
/// position, described by any point on the plane, is not stored directly.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Plane {
    local_normal: DVec3,
    // cache the normal in global space as planes are likely to be fixed after their initial
    // positioning.
    normal: DVec3,
}

impl Plane {
    /// Creates a new infinite plane. The plane normal is equivalent to the y-axis in the local
    /// body space of the plane and the given PhysTransform is used to cache its normal in
    /// global space. The update function must then be run to update the cache if the Plane moves.
    pub fn new(transform: &PhysTransform) -> Self {
        let mut result = Self {
            local_normal: DVec3::Y,
            normal: DVec3::ZERO,
        };
        result.update(transform);
        result
    }

    /// Returns a DVec3 representing the normal of the plane in body space.
    pub fn normal_in_body_space(&self) -> DVec3 {
        self.local_normal
    }

    /// Returns a DVec3 representing the normal of the plane in global space. The 'update' method
    /// must be run before retrieving the normal if the plane has moved, otherwise it will be out
    /// of date.
    pub fn normal(&self) -> DVec3 {
        self.normal
    }

    /// Updates derived data. Must be run after the plane has been moved to ensure accuracy of the
    /// normal in global coords.
    pub fn update(&mut self, transform: &PhysTransform) {
        self.normal = transform.get_direction_in_global_space(self.local_normal);
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
        let t = self.local_normal.dot(target_local);  // assume unit normal so n.n = 1.

        let result_local = target_local - t * self.local_normal;

        // Convert the result back into global coords.
        transform.get_point_in_global_space(result_local)
    }

    /// Calculates and returns the shortest distance between the Plane, with the given transform,
    /// and the target point in global coords.
    fn shortest_distance_to(&self, transform: &PhysTransform, target: DVec3) -> f64 {
        let target_local = transform.get_point_in_local_space(target);
        self.local_normal.dot(target_local)
    }
}
