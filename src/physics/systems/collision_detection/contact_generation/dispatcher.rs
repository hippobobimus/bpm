use bevy::prelude::Entity;

use crate::{
    physics::components::{
        Contact,
        PhysTransform,
    },
    physics::shapes::{
        CollisionPrimative,
        Cuboid,
        Plane,
        Sphere,
    },
    physics::systems::collision_detection::contact_generation::contact_generators,
};

/// Generates contacts between two shapes with the CollisionPrimative trait by downcasting to the
/// concrete shape type known at runtime and dispatching to the appropriate contact generation
/// function.
pub fn generate_primative_contacts(
    ent_a: Entity,
    ent_b: Entity,
    a: &Box<dyn CollisionPrimative>,
    b: &Box<dyn CollisionPrimative>,
    transform_a: &PhysTransform,
    transform_b: &PhysTransform,
) -> Option<Vec<Contact>> {
    let a_is_sphere =  a.is::<Sphere>();
    let b_is_sphere =  b.is::<Sphere>();

    let a_is_cuboid = a.is::<Cuboid>();
    let b_is_cuboid = b.is::<Cuboid>();

    if a_is_sphere && b_is_sphere {
        return contact_generators::sphere_and_sphere(
            ent_a,
            ent_b,
            a.downcast_ref::<Sphere>().unwrap(),
            b.downcast_ref::<Sphere>().unwrap(),
            transform_a,
            transform_b,
        ).map(|c| vec![c])
    }
    if a_is_cuboid && b_is_cuboid {
        return contact_generators::cuboid_and_cuboid(
            ent_a,
            ent_b,
            a.downcast_ref::<Cuboid>().unwrap(),
            b.downcast_ref::<Cuboid>().unwrap(),
            transform_a,
            transform_b,
        ).map(|c| vec![c])
    }
    if a_is_sphere && b_is_cuboid {
        return contact_generators::sphere_and_cuboid(
            ent_a,
            ent_b,
            a.downcast_ref::<Sphere>().unwrap(),
            b.downcast_ref::<Cuboid>().unwrap(),
            transform_a,
            transform_b,
        ).map(|c| vec![c])
    }
    if a_is_cuboid && b_is_sphere {
        return contact_generators::sphere_and_cuboid(
            ent_b,
            ent_a,
            b.downcast_ref::<Sphere>().unwrap(),
            a.downcast_ref::<Cuboid>().unwrap(),
            transform_b,
            transform_a,
        ).map(|c| vec![c])
    }

    None
}

/// Generates contacts between a half-space boundary represented by a Plane and a
/// CollisionPrimative.
pub fn generate_boundary_contacts(
    ent_other: Entity,
    bnd: &Plane,
    other: &Box<dyn CollisionPrimative>,
    transform_bnd: &PhysTransform,
    transform_other: &PhysTransform,
) -> Option<Vec<Contact>> {
    // Downcast at runtime to determine concrete type of CollisionPrimative.
    let other_is_cuboid = other.is::<Cuboid>();
    let other_is_sphere = other.is::<Sphere>();

    if other_is_sphere {
        return contact_generators::half_space_and_sphere(
            ent_other,
            bnd,
            other.downcast_ref::<Sphere>().unwrap(),
            transform_other,
            transform_bnd,
        ).map(|c| vec![c])
    }
    if other_is_cuboid {
        return contact_generators::half_space_and_cuboid(
            ent_other,
            bnd,
            other.downcast_ref::<Cuboid>().unwrap(),
            transform_other,
            transform_bnd,
        )
    }

    None
}
