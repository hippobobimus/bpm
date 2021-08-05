use crate::physics::shapes::{CollisionPrimative, Sphere};

pub struct Collider(pub Box<dyn CollisionPrimative>);

impl Collider {
    pub fn new<T: CollisionPrimative>(primative: T) -> Self {
        Self(Box::new(primative))
    }
}

impl Default for Collider {
    fn default() -> Self {
        Self::new(Sphere::new(1.0))
    }
}
