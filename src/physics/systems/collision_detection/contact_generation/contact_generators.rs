use bevy::math::DVec3;

use crate::{
    physics::shapes::*,
    physics::systems::collision_detection::contact_generation::contact::Contact,
};

// Boolean queries

/// Evaluates whether the axis-aligned bounding box centred at the given position and the plane
/// containing the given point intersect.
pub fn aabb_and_plane_in_contact(a: &Aabb3D, a_pos: DVec3, p: &Plane, p_pos: DVec3) -> bool {
    // Test separating axis that intersects aabb centre and is parallel to plane normal;
    // L(t) = a.centre + t * p.normal.

    // Calculate projection radius of aabb onto L.
    let r = a.extents().x * p.normal().x.abs() + a.extents().y * p.normal().y.abs();

    // distance of aabb centre from plane.
    let dist = p.shortest_distance_to(p_pos, a_pos);

    // consider the negative half-space behind the plane to be solid.
    dist <= r
}

// Generators

///
pub fn sphere_and_sphere(s1: &Sphere, pos_1: DVec3, s2: &Sphere, pos_2: DVec3) -> Option<Contact> {
    let midline = pos_1 - pos_2;
    let length = midline.length();

    if length <= 0.0 || length >= s1.radius() + s2.radius() {
        return None;
    }

    // already verified not of length 0 so can normalize.
    let normal = midline.normalize();
    let point = pos_1 + midline * 0.5;
    let penetration = (s1.radius() + s2.radius()) - length;

    Some(Contact { normal, penetration, point })
}

pub fn sphere_and_half_space(s: &Sphere, s_pos: DVec3, p: &Plane, p_pos: DVec3) -> Option<Contact> {
    let d = p.shortest_distance_to(p_pos, s_pos);

    if d >= s.radius() {
        return None;
    }

    let normal = *p.normal();
    let penetration = s.radius() - d;
    let point = p.closest_point_to(p_pos, s_pos);

    Some(Contact { normal, penetration, point })
}
