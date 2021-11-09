use bevy::{
    prelude::Entity,
    math::DVec3,
};

#[derive(Debug)]
/// Describes a contact between two separate bodies associated with two entities. Contains the two
/// Entitys involved in the contact (unless one of the Entitys is an immovable plane, in which case
/// just the body that impacts the plane is referenced), the point of contact, amount of
/// inter-penetration, the contact normal vector and the contact point(s) relative to each body.
pub struct Contact {
    pub entities: Vec<Entity>,
    pub normal: DVec3,
    pub penetration: f64,
    pub point: DVec3,
    pub relative_points: Vec<DVec3>,
}
