use bevy::math::DVec3;

use crate::physics::components::PhysTransform;

/// Collidable shapes must be able to determine the closest point on their area to a given target
/// and the absolute distance between those points.
pub trait Collidable: std::fmt::Debug {
    /// Returns the closest point on the shape with the given transform to the given target point
    /// in global coords.
    fn closest_point_to(&self, transform: &PhysTransform, target: DVec3) -> DVec3;

    /// Returns the distance from the closest point on the shape to the given target point in
    /// global coords.
    fn shortest_distance_to(&self, transform: &PhysTransform, point: DVec3) -> f64;
}
