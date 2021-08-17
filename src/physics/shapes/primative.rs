use downcast_rs::{
    Downcast,
    impl_downcast,
};

use crate::physics::shapes::Sphere;

pub trait CollisionPrimative: std::fmt::Debug + Downcast + Send + Sync {
    fn bounding_sphere(&self) -> &Sphere;
}

impl_downcast!(CollisionPrimative);
