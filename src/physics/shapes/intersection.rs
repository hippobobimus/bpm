use bevy::math::DVec3;

use crate::physics::shapes::*;

/// Evaluates whether the axis-aligned bounding box centred at the given position and the plane
/// containing the given point intersect.
pub fn aabb_plane_are_intersecting(a: &Aabb3D, a_pos: DVec3, p: &Plane, p_pos: DVec3) -> bool {
    // Test separating axis that intersects aabb centre and is parallel to plane normal;
    // L(t) = a.centre + t * p.normal.

    // Calculate projection radius of aabb onto L.
    let r = a.extents().x * p.normal().x.abs() + a.extents().y * p.normal().y.abs();

    // distance of aabb centre from plane.
    let dist = p.shortest_dist_to(p_pos, a_pos);

    // consider the negative half-space behind the plane to be solid.
    dist <= r
}
