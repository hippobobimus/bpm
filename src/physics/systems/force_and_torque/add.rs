use bevy::math::DVec3;

use crate::physics::components::{
    Force,
    PhysTransform,
    Torque,
};

/// Updates the force accumulator based on a given force with a direction that intersects the
/// centre of mass of the body (i.e. no torques are generated). The force is given in global
/// coords.
pub fn add_force(
    force: DVec3,
    force_accum: &mut Force,
) {
    force_accum.add(force);
}

/// Updates the force and torque accumulators based on a given force applied at a given point.
/// The point is in body coords and the direction of the force is given in global coords.
pub fn add_force_at_body_point(
    force: DVec3,
    mut point: DVec3,
    body_transform: &PhysTransform,
    force_accum: &mut Force,
    torque_accum: &mut Torque
) {
    // convert point to global coords.
    point = body_transform.get_point_in_global_space(point);

    add_force_at_point(
        force,
        point,
        body_transform.translation(),
        force_accum,
        torque_accum,
    );
}

/// Updates the force and torque accumulators based on a given force applied at a given point.
/// The point and the direction of the force are given in body coords.
pub fn add_body_force_at_body_point(
    mut force: DVec3,
    mut point: DVec3,
    body_transform: &PhysTransform,
    force_accum: &mut Force,
    torque_accum: &mut Torque
) {
    // convert force and point to global coords.
    force = body_transform.get_direction_in_global_space(force);
    point = body_transform.get_point_in_global_space(point);

    add_force_at_point(
        force,
        point,
        body_transform.translation(),
        force_accum,
        torque_accum,
    );
}

/// Updates the force and torque accumulators based on a given force applied at a given point. The
/// force and point are given in global coords.
pub fn add_force_at_point(
    force: DVec3,
    point: DVec3,
    centre_of_mass: DVec3,
    force_accum: &mut Force,
    torque_accum: &mut Torque
) {
    // get vector from centre of mass to force application point.
    let d = point - centre_of_mass;

    force_accum.add(force);
    torque_accum.add(d.cross(force));
}
