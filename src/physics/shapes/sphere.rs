use crate::physics::shapes::Collider;

/// A sphere described by its radius only. The centre position is not stored directly.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    radius: f64,
}

impl Sphere {
    /// Creates a new sphere with the given radius.
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }

    /// Returns the radius of the sphere.
    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Collider for Sphere {}
