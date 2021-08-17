use bevy::math::DVec3;

use crate::physics::shapes::Plane;

/// A component that allows an entity to participate in collision physics by assigning a plane to
/// it that acts as a rigid half-space.
#[derive(Debug)]
pub struct BoundaryCollider(pub Plane);

impl BoundaryCollider {
    /// Creates a new BoundaryCollider with the given Plane as a half-space collision boundary.
    pub fn new(plane: Plane) -> Self {
        Self(plane)
    }
}

impl Default for BoundaryCollider {
    /// Returns a BoundaryCollider formed by a plane with a normal equal to the positive y-axis.
    fn default() -> Self {
        Self::new(Plane::new(DVec3::Y))
    }
}
