use bevy::math::DVec3;

/// Shapes must be able to determine the closest point on their area to a given target and the
/// absolute distance between those points.
pub trait Collidable: std::fmt::Debug {
    /// Returns the closest point on the shape to the given target point.
    fn closest_point_to(&self, centre: DVec3, target: DVec3) -> DVec3;

    /// Returns the distance from the closest point on the shape to the given target point.
    fn shortest_dist_to(&self, centre: DVec3, point: DVec3) -> f64;
}
