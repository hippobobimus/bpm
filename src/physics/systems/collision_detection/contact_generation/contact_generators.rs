use bevy::math::DVec3;

use crate::{
    physics::components::PhysTransform,
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

/// Evaluates two spheres for intersection, generating a Contact if they are found to be
/// intersecting.
pub fn sphere_and_sphere(
    s1: &Sphere,
    s2: &Sphere,
    s1_transform: &PhysTransform,
    s2_transform: &PhysTransform,
) -> Option<Contact> {
    let s1_centre = s1_transform.translation();
    let s2_centre = s2_transform.translation();
    let midline = s2_centre - s1_centre;
    let d = midline.length();
    let sum_of_radii = s1.radius() + s2.radius();

    if d <= 0.0 || d >= sum_of_radii {
        return None;
    }

    // already verified not of length 0 so can normalize.
    let normal = midline.normalize();
    let point = s1_centre + midline * 0.5;
    let penetration = sum_of_radii - d;

    Some(Contact { normal, penetration, point })
}

/// Evaluates a sphere and half-space for intersection, generating a Contact if they are found to
/// be intersecting.
pub fn sphere_and_half_space(
    s: &Sphere,
    p: &Plane,
    s_transform: &PhysTransform,
    p_transform: &PhysTransform
) -> Option<Contact> {
    let s_centre = s_transform.translation();
    let p_point = p_transform.translation();
    let d = p.shortest_distance_to(p_point, s_centre);

    if d >= s.radius() {
        return None;
    }

    let normal = *p.normal();
    let penetration = s.radius() - d;
    let point = p.closest_point_to(p_point, s_centre);

    Some(Contact { normal, penetration, point })
}

/// Evaluates a cuboid and sphere for intersection, generating a Contact if they are found to be
/// intersecting.
pub fn cuboid_and_sphere(
    c: &Cuboid,
    s: &Sphere,
    c_transform: &PhysTransform,
    s_transform: &PhysTransform,
) -> Option<Contact> {
    let c_centre = c_transform.translation();
    let s_centre = s_transform.translation();

    let closest_point = c.closest_point_to(c_centre, s_centre);

    let shortest_dist = (closest_point - s_transform.translation()).length() - s.radius();

    // Check whether they are in contact.
    if shortest_dist > 0.0 {
        return None;
    }

    // normal from sphere centre to closest point on cuboid.
    let normal = (closest_point - s_centre).normalize();

    Some(Contact {
        normal,
        penetration: shortest_dist.abs(),
        point: closest_point,
    })
}

/// Evaluates a cuboid and half-space for intersection, generating Contact(s) if they are found to
/// be intersecting.
pub fn cuboid_and_half_space(
    c: &Cuboid,
    p: &Plane,
    c_transform: &PhysTransform,
    p_transform: &PhysTransform,
) -> Option<Vec<Contact>> {
    let mut contacts = vec![];
    let p_pos = p_transform.translation();

    for vertex_position in c.vertices(c_transform).iter() {
        let vertex_dist = p.shortest_distance_to(p_pos, *vertex_position);
        if vertex_dist <= 0.0 {
            let normal = *p.normal();
            // contact point is mid-point between vertex and plane.
            let point = *vertex_position + normal * (vertex_dist.abs() * 0.5);
            let penetration = vertex_dist.abs();

            contacts.push(Contact {normal, penetration, point });
        }
    }

    if contacts.is_empty() {
        return None;
    }
    Some(contacts)
}

/// Evaluates a two cuboids for intersection, generating a Contact if they are found to
/// be intersecting.
pub fn cuboid_and_cuboid(
    c1: &Cuboid,
    c2: &Cuboid,
    c1_transform: &PhysTransform,
    c2_transform: &PhysTransform,
) -> Option<Contact> {
    let mut penetration = f64::MAX;
    let mut case = usize::MAX;

    let centre_to_centre = c2_transform.translation() - c1_transform.translation();

    // Check face axes for cuboid 1. (cases 0..3)
    for i in 0..3 {
        let axis = c1_transform.axis(i).normalize();
        let case_idx = i;

        // If no overlap is found then we have a separating axis and there are no contacts (so
        // return None). If an overlap smaller than the current best is found then the best is
        // updated.
        if !check_axis(c1, c2, c1_transform, c2_transform, axis, centre_to_centre, case_idx,
                       &mut penetration, &mut case) {
            return None;
        }
    }

    // Check face axes for cuboid 2. (cases 3..6)
    for i in 0..3 {
        let axis = c2_transform.axis(i).normalize();
        let case_idx = i + 3;

        if !check_axis(c1, c2, c1_transform, c2_transform, axis, centre_to_centre, case_idx,
                       &mut penetration, &mut case) {
            return None;
        }
    }

    // Check edge-edge axes. (cases 6..16)
    for i in 0..2 {
        for j in 0..2 {
            let axis = c1_transform.axis(i).cross(c2_transform.axis(j));
            let case_idx = 3 * i + j + 6;

            // Skip axes that have been generated from near-parallel edges.
            if axis.length_squared() < 0.001 { continue; }
            axis.normalize();

            if !check_axis(c1, c2, c1_transform, c2_transform, axis, centre_to_centre, case_idx,
                           &mut penetration, &mut case) {
                return None;
            }
        }
    }

    // At this point we must have found a penetrating case, otherwise an error has occurred.
    assert!(case != usize::MAX);

    let contact = if case < 3 {
        // face of cuboid 1, vertex of cuboid 2.
        let axis_idx = case;
        calc_cuboid_face_vertex_contact(c2, c1_transform, c2_transform, axis_idx,
                                        centre_to_centre, penetration)
    } else if case < 6 {
        // face of cuboid 2, vertex of cuboid 1.
        let axis_idx = case - 3;
        calc_cuboid_face_vertex_contact(c1, c2_transform, c1_transform, axis_idx,
                                        -centre_to_centre, penetration)
    } else if case < 16 {
        // edge-edge.
        let axis_idx_1 = (case - 6) / 3;
        let axis_idx_2 = (case - 6) % 3;
        calc_cuboid_edge_edge_contact(c1, c2, c1_transform, c2_transform, axis_idx_1, axis_idx_2,
                                      centre_to_centre, penetration)
    } else {
        panic!("incorrect case enumeration ({})!", case);
    };

    Some(contact)
}

/// Generates the contact data when edges from each cuboid are in contact.
fn calc_cuboid_edge_edge_contact(
    c1: &Cuboid,
    c2: &Cuboid,
    c1_transform: &PhysTransform,
    c2_transform: &PhysTransform,
    axis_idx_1: usize,
    axis_idx_2: usize,
    centre_to_centre: DVec3,
    penetration: f64,
) -> Contact {
    // Find axis between edges.
    let c1_axis = c1_transform.axis(axis_idx_1);
    let c2_axis = c2_transform.axis(axis_idx_2);
    let mut normal = c1_axis.cross(c2_axis).normalize();

    // Make sure axis points from cuboid 1 to cuboid 2.
    if normal.dot(centre_to_centre) > 0.0 {
        normal = normal * -1.0;
    }

    // Find the edges. 4 parallel possibilities for each cuboid. Find the centre point fo the edge
    // - it must have a 0 component in the collision axis direction, then project the cuboid
    // extents on to the axis to find the closest.
    let mut c1_point = c1.extents();
    let mut c2_point = c2.extents();
    for i in 0..2 {
        if i == axis_idx_1 {
            c1_point[i] = 0.0;
        } else if c1_transform.axis(i).dot(normal) > 0.0 { // TODO check, this seems backwards??
            c1_point[i] = -c1_point[i];
        }

        if i == axis_idx_2 {
            c2_point[i] = 0.0;
        } else if c2_transform.axis(i).dot(normal) < 0.0 {
            c2_point[i] = -c2_point[i];
        }
    }

    // Transform into global coords.
    c1_point = c1_transform.mul_vec3(c1_point);
    c2_point = c2_transform.mul_vec3(c2_point);

    // Get closest point between the two edges.
    let contact_point = calc_point_between_skew_lines(c1_axis, c2_axis, c1_point, c2_point);

    Contact {
        normal,
        penetration,
        point: contact_point,
    }
}

/// Returns the point halfway between the points of closest approach on a pair of skew lines, given
/// a point and direction for each line.
fn calc_point_between_skew_lines(
    d1: DVec3,
    d2: DVec3,
    p1: DVec3,
    p2: DVec3,
) -> DVec3 {
    // Let closest points on each line be;
    //
    // Line 1:  q1 = p1 + a * d1
    // Line 2:  q2 = p2 + b * d2
    //
    // The vector (q2 - q1) must be perpendicular to both lines. Therefore;
    //
    // (q2 - q1).d1 = 0
    // (q2 - q1).d2 = 0
    //
    // Substituting for q1 and q2, then solving for a and b gives:
    //
    // a = (d1.d2 * (p2 - p1).d2 - |d2|^2 * (p2 - p1).d1) / (|d1.d2|^2 - |d1|^2 * |d2|^2)
    //
    // b = (|d1|^2 * (p2 - p1).d2 - d1.d2 * (p2 - p1).d1) / (|d1.d2|^2 - |d1|^2 * |d2|^2)
    //
    let s = p2 - p1;
    let d1_sq = d1.length_squared();
    let d2_sq = d2.length_squared();
    let d1_dot_q = d1.dot(s);
    let d2_dot_q = d2.dot(s);
    let d1_dot_d2 = d1.dot(d2);
    let denom = d1_dot_d2 * d1_dot_d2 - d1_sq * d2_sq;

    let a = (d1_dot_d2 * d2_dot_q - d2_sq * d1_dot_q) / denom;
    let b = (d1_sq * d2_dot_q - d1_dot_d2 * d1_dot_q) / denom;

    // nearest points.
    let q1 = p1 + a * d1;
    let q2 = p2 + b * d2;

    // midpoint.
    (q1 + q2) * 0.5
}

/// Generates the contact data when a vertex from cuboid 2 is in contact with a face of cuboid 1.
fn calc_cuboid_face_vertex_contact(
    c2: &Cuboid,
    c1_transform: &PhysTransform,
    c2_transform: &PhysTransform,
    axis_idx: usize,
    centre_to_centre: DVec3,
    penetration: f64,
) -> Contact {
    // Find contact face. We know it is a face of cuboid 1 on the collision axis, determine which
    // one and flip the axis to find the normal if necessary.
    let mut normal = c1_transform.axis(axis_idx);
    if normal.dot(centre_to_centre) > 0.0 {
        normal = normal * -1.0;
    }

    // Find contacting vertex of cuboid 2.
    let mut vertex = c2.extents();
    if c2_transform.axis(0).dot(normal) < 0.0 {
        vertex.x = -vertex.x;
    }
    if c2_transform.axis(1).dot(normal) < 0.0 {
        vertex.y = -vertex.y;
    }
    if c2_transform.axis(2).dot(normal) < 0.0 {
        vertex.z = -vertex.z;
    }

    // Convert to world coords.
    vertex = c2_transform.mul_vec3(vertex);

    Contact {
        normal,
        penetration,
        point: vertex,
    }
}

/// Finds the penetration of the projections of the two cuboids on the given axis. If a penetration
/// is found it is compared to the current penetration value and, if it is lower, replaces it and
/// marks itself as the best case. A boolean is returned indicating whether a penetration value was
/// or was not found.
fn check_axis(
    c1: &Cuboid,
    c2: &Cuboid,
    c1_transform: &PhysTransform,
    c2_transform: &PhysTransform,
    axis: DVec3,
    centre_to_centre: DVec3,
    case_idx: usize,
    penetration: &mut f64,
    case: &mut usize,
) -> bool {
    match penetration_on_axis(c1, c2, c1_transform, c2_transform, axis, centre_to_centre) {
        Some(new_pen) if new_pen < *penetration => {
            *penetration = new_pen;
            *case = case_idx;
        },
        None => return false,
        _ => (),
    }
    true
}

/// Returns the overlap of the two given cuboids when projected onto the given axis. If the cuboids
/// are not overlapping, returns None.
fn penetration_on_axis(
    c1: &Cuboid,
    c2: &Cuboid,
    c1_transform: &PhysTransform,
    c2_transform: &PhysTransform,
    axis: DVec3,
    centre_to_centre: DVec3,
) -> Option<f64> {
    let distance = centre_to_centre.dot(axis);

    let overlap = c1.project_onto_axis(c1_transform, axis) +
        c2.project_onto_axis(c2_transform, axis) - distance;

    if overlap < 0.0 {
        return None;
    }
    Some(overlap)
}

#[cfg(test)]
mod test {
    use super::*;

    use bevy::math::DQuat;

    const EPSILON: f64 = 0.000001;

    #[test]
    fn test_sphere_and_sphere() {
        let r1 = 1.0;
        let r2 = 1.0;

        let s1 = Sphere::new(r1);
        let s2 = Sphere::new(r2);

        let s1_transform = PhysTransform::IDENTITY;

        // NO PENETRATION.
        let s2_transform = PhysTransform::from_translation(
            DVec3::new(0.0, r1 + r2 + 0.001, 0.0),
        );
        let contact = sphere_and_sphere(&s1, &s2, &s1_transform, &s2_transform);

        assert!(contact.is_none());

        // PENETRATION.
        let expected_penetration = 0.0005;
        let expected_contact_point = DVec3::new(0.0, 0.99975, 0.0);
        let expected_normal = DVec3::Y;

        let s2_transform = PhysTransform::from_translation(
            DVec3::new(0.0, r1 + r2 - expected_penetration, 0.0),
        );
        let contact = sphere_and_sphere(&s1, &s2, &s1_transform, &s2_transform).unwrap();

        println!("Contact: {:?}", contact);

        assert!((expected_normal.x - contact.normal.x).abs() < EPSILON);
        assert!((expected_normal.y - contact.normal.y).abs() < EPSILON);
        assert!((expected_normal.z - contact.normal.z).abs() < EPSILON);
        assert!((expected_penetration - contact.penetration).abs() < EPSILON);
        assert!((expected_contact_point.x - contact.point.x).abs() < EPSILON);
        assert!((expected_contact_point.y - contact.point.y).abs() < EPSILON);
        assert!((expected_contact_point.z - contact.point.z).abs() < EPSILON);
    }

    #[test]
    fn test_sphere_and_half_space() {
    }

    #[test]
    fn test_cuboid_and_sphere() {
    }

    #[test]
    fn test_cuboid_and_half_space() {
        let extents = DVec3::new(3.0, 3.0, 4.0);
        let c = Cuboid::new(extents);

        let normal = DVec3::Y;
        let p = Plane::new(normal);
        let p_transform = PhysTransform::IDENTITY;

        // NO PENETRATING VERTICES
        let c_transform = PhysTransform::from_translation(
            DVec3::new(0.0, 4.0, 0.0),
        );
        let contacts = cuboid_and_half_space(&c, &p, &c_transform, &p_transform);
        assert!(contacts.is_none());

        // FOUR PENETRATING VERTICES.
        let c_transform = PhysTransform::from_translation(
            DVec3::new(0.0, 1.0, 0.0),
        );
        let expected_penetration = 2.0;

        let contacts = cuboid_and_half_space(&c, &p, &c_transform, &p_transform).unwrap();

        assert_eq!(4, contacts.len());
        for contact in contacts.iter() {
            assert_eq!(normal, contact.normal);
            assert_eq!(expected_penetration, contact.penetration);
            assert!((contact.point.y - -1.0).abs() < EPSILON);
        }
        assert!((contacts[0].point.x - 3.0).abs() < EPSILON);
        assert!((contacts[0].point.z - 4.0).abs() < EPSILON);

        assert!((contacts[1].point.x - 3.0).abs() < EPSILON);
        assert!((contacts[1].point.z - -4.0).abs() < EPSILON);

        assert!((contacts[2].point.x - -3.0).abs() < EPSILON);
        assert!((contacts[2].point.z - 4.0).abs() < EPSILON);

        assert!((contacts[3].point.x - -3.0).abs() < EPSILON);
        assert!((contacts[3].point.z - -4.0).abs() < EPSILON);

        // TWO PENETRATING VERTICES.
        let expected_penetration = 1.0;
        let c_transform = PhysTransform::from_rotation_translation(
            DQuat::from_rotation_z(-0.25 * std::f64::consts::PI),
            DVec3::new(0.0, 3.0 * 2.0_f64.sqrt() - expected_penetration, 0.0),
        );

        let contacts = cuboid_and_half_space(&c, &p, &c_transform, &p_transform).unwrap();

        assert_eq!(2, contacts.len());
        for contact in contacts.iter() {
            assert_eq!(normal, contact.normal);
            assert_eq!(expected_penetration, contact.penetration);
            assert!((contact.point.x - 0.0).abs() < EPSILON);
            assert!((contact.point.y - -0.5).abs() < EPSILON);
        }
        assert!((contacts[0].point.z - 4.0).abs() < EPSILON);
        assert!((contacts[1].point.z - -4.0).abs() < EPSILON);

        // ONE PENETRATING VERTEX.
        let extents = DVec3::new(3.0, 3.0, 3.0);
        let c = Cuboid::new(extents);

        let expected_penetration = 0.1;
        let rotation = DQuat::from_axis_angle(
            DVec3::new(1.0, 0.0, -1.0).normalize(),
            (1.0 / 3.0_f64.sqrt()).acos(),
        );
        let translation = DVec3::new(0.0, 3.0 * 3.0_f64.sqrt() - expected_penetration, 0.0);
        let c_transform = PhysTransform::from_rotation_translation(rotation, translation);

        let contacts = cuboid_and_half_space(&c, &p, &c_transform, &p_transform).unwrap();

        assert_eq!(1, contacts.len());
        assert_eq!(normal, contacts[0].normal);
        assert!((contacts[0].penetration - expected_penetration).abs() < EPSILON);
        assert!((contacts[0].point.x - 0.0).abs() < EPSILON);
        assert!((contacts[0].point.y - -expected_penetration / 2.0).abs() < EPSILON);
        assert!((contacts[0].point.z - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_cuboid_and_cuboid() {
    }
}
