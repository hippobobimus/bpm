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
) -> Option<Contact> {
    let a_is_sphere =  a.is::<Sphere>();
    let b_is_sphere =  b.is::<Sphere>();

    if a_is_sphere && b_is_sphere {
        return contact_generators::sphere_and_sphere(
            a.downcast_ref::<Sphere>().unwrap(),
            transform_a.translation(),
            b.downcast_ref::<Sphere>().unwrap(),
            transform_b.translation(),
        )
    }

    None
}
