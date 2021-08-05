pub mod aabb;
pub mod collidable;
pub mod cuboid;
//pub mod intersection;
pub mod plane;
pub mod primative;
pub mod sphere;

pub use crate::physics::shapes::{
    aabb::Aabb3D,
    collidable::Collidable,
    cuboid::Cuboid,
    plane::Plane,
    primative::CollisionPrimative,
    sphere::Sphere,
    //intersection::*,
};
