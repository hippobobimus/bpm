use crate::physics::shapes::{CollisionPrimative, Sphere};

/// A component that allows an entity to participate in collision physics by assigning a primative
/// collision shape to it.
#[derive(Debug)]
pub struct Collider(pub Box<dyn CollisionPrimative>);

impl Collider {
    /// Creates a new Collider with the given primative shape.
    pub fn new<T: CollisionPrimative>(primative: T) -> Self {
        Self(Box::new(primative))
    }
}

impl Default for Collider {
    /// Returns a Collider with a radius 1.0 sphere as its primative shape.
    fn default() -> Self {
        Self::new(Sphere::new(1.0))
    }
}
