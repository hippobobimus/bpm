pub mod aabb;
pub mod collidable;
pub mod intersection;
pub mod plane;
pub mod sphere;

pub use crate::physics::shapes::{
    aabb::Aabb3D,
    collidable::Collidable,
    plane::Plane,
    sphere::Sphere,
    intersection::*,
};
