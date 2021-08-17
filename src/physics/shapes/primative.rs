use downcast_rs::{
    Downcast,
    impl_downcast,
};

use crate::physics::shapes::Sphere;

/// Primative shapes that can take part in collision physics,
pub trait CollisionPrimative: std::fmt::Debug + Downcast + Send + Sync {
    /// Returns a Sphere that contains the primative shape in its entirety.
    fn bounding_sphere(&self) -> &Sphere;
}

// implement downcasting to the concrete type of the primative shape for dispatching to relevant
// contact generation functions.
impl_downcast!(CollisionPrimative);
