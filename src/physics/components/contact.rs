use bevy::{
    prelude::Entity,
    math::DVec3,
};

#[derive(Debug)]
/// Describes a contact between two separate bodies associated with two entities. Contains the two
/// Entitys involved in the contact, the point of contact, amount of inter-penetration and the
/// contact normal vector.
pub struct Contact {
    pub entities: (Option<Entity>, Option<Entity>),
    pub normal: DVec3,
    pub penetration: f64,
    pub point: DVec3,
}
