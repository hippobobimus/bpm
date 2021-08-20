use crate::{
    physics::shapes::Plane,
    physics::components::PhysTransform,
};

/// A component that allows an entity to participate in collision physics by assigning a plane to
/// it that acts as a rigid half-space.
#[derive(Debug)]
pub struct BoundaryCollider(pub Plane);

impl BoundaryCollider {
    /// Creates a new BoundaryCollider based on the given PhysTransform applied to an x-z plane with
    /// a normal in the positive y-axis.
    pub fn new(transform: &PhysTransform) -> Self {
        Self(Plane::new(transform))
    }
}

impl Default for BoundaryCollider {
    /// Returns a BoundaryCollider formed by an x-z plane with a normal equal to the positive
    /// y-axis.
    fn default() -> Self {
        Self::new(&PhysTransform::IDENTITY)
    }
}
