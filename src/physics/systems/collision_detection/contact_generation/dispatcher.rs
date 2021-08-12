use crate::{
    physics::prelude::*,
    physics::systems::collision_detection::contact_generation::contact::Contact,
    physics::systems::collision_detection::contact_generation::contact_generators,
    physics::shapes::CollisionPrimative,
};

/// Generates contacts between two shapes with the CollisionPrimative trait by downcasting to the
/// concrete shape type known at runtime and dispatching to the appropriate contact generation
/// function.
pub fn generate_contacts(
    a: &Box<dyn CollisionPrimative>,
    b: &Box<dyn CollisionPrimative>,
    transform_a: &PhysTransform,
    transform_b: &PhysTransform,
) -> Option<Vec<Contact>> {
    let a_is_sphere =  a.is::<Sphere>();
    let b_is_sphere =  b.is::<Sphere>();

    let a_is_plane = a.is::<Plane>();
    let b_is_plane = b.is::<Plane>();

    let a_is_cuboid = a.is::<Cuboid>();
    let b_is_cuboid = b.is::<Cuboid>();

    if a_is_sphere && b_is_sphere {
        return contact_generators::sphere_and_sphere(
            a.downcast_ref::<Sphere>().unwrap(),
            b.downcast_ref::<Sphere>().unwrap(),
            transform_a,
            transform_b,
        ).map(|c| vec![c])
    }
    if a_is_cuboid && b_is_cuboid {
        return contact_generators::cuboid_and_cuboid(
            a.downcast_ref::<Cuboid>().unwrap(),
            b.downcast_ref::<Cuboid>().unwrap(),
            transform_a,
            transform_b,
        ).map(|c| vec![c])
    }
    if a_is_cuboid && b_is_sphere {
        return contact_generators::cuboid_and_sphere(
            a.downcast_ref::<Cuboid>().unwrap(),
            b.downcast_ref::<Sphere>().unwrap(),
            transform_a,
            transform_b,
        ).map(|c| vec![c])
    }
    if a_is_sphere && b_is_cuboid {
        return contact_generators::cuboid_and_sphere(
            b.downcast_ref::<Cuboid>().unwrap(),
            a.downcast_ref::<Sphere>().unwrap(),
            transform_b,
            transform_a,
        ).map(|c| vec![c])
    }
    if a_is_sphere && b_is_plane {
        return contact_generators::sphere_and_half_space(
            a.downcast_ref::<Sphere>().unwrap(),
            b.downcast_ref::<Plane>().unwrap(),
            transform_a,
            transform_b,
        ).map(|c| vec![c])
    }
    if a_is_plane && b_is_sphere {
        return contact_generators::sphere_and_half_space(
            b.downcast_ref::<Sphere>().unwrap(),
            a.downcast_ref::<Plane>().unwrap(),
            transform_b,
            transform_a,
        ).map(|c| vec![c])
    }
    if a_is_cuboid && b_is_plane {
        return contact_generators::cuboid_and_half_space(
            a.downcast_ref::<Cuboid>().unwrap(),
            b.downcast_ref::<Plane>().unwrap(),
            transform_a,
            transform_b,
        )
    }
    if a_is_plane && b_is_cuboid {
        return contact_generators::cuboid_and_half_space(
            b.downcast_ref::<Cuboid>().unwrap(),
            a.downcast_ref::<Plane>().unwrap(),
            transform_b,
            transform_a,
        )
    }

    None
}
