use crate::physics::shapes::{Collider, Sphere};

pub struct CollisionPrimative(pub Box<dyn Collider>);

impl CollisionPrimative {
    pub fn new<T: Collider>(primative: T) -> Self {
        Self(Box::new(primative))
    }
}

impl Default for CollisionPrimative {
    fn default() -> Self {
        Self::new(Sphere::new(1.0))
    }
}
