use bevy::{
    prelude::*,
    math::DVec3,
};

use crate::{
    physics::{Drag, Force, Gravity, Mass, Thrust, Torque, Velocity},
};

/// A system that calculates and accumulates various forces and associated torques applied on a
/// body.
pub fn force_accumulation(
    mut q: QuerySet<(
        Query<(&mut Force, &mut Torque)>,
        Query<(&Drag, &mut Force, &Velocity)>,
        Query<(&Gravity, &mut Force, &Mass)>,
        Query<(&Thrust, &mut Force)>,
    )>,
) {
    // Zero the force and torque accumulators.
    for (mut f, mut tq) in q.q0_mut().iter_mut() {
        f.reset();
        tq.reset();
    }

    // Apply force generators.
    for (drag, mut f, v) in q.q1_mut().iter_mut() {
        f.add(drag.force(*v.vector()));
    }
    for (gravity, mut f, m) in q.q2_mut().iter_mut() {
        // ensure the mass is not 0 or infinite (or subnormal/NaN).
        if !m.is_normal() { break };
        f.add(gravity.force(m.value()));
    }
    for (thrust, mut f) in q.q3_mut().iter_mut() {
        f.add(*thrust.force());
    }

    // TODO apply torques
}

/// Updates the force accumulator based on a given force with a direction that intersects the
/// centre of mass of the body (i.e. no torques are generated).
fn add_force(
    force: DVec3,
    force_accum: &mut Force,
) {
    force_accum.add(force);
}

/// Updates the force and torque accumulators based on a given force applied at a given point
/// relative to the body's centre of mass.
fn add_force_at_body_point(
    force: DVec3,
    point: DVec3,
    force_accum: &mut Force,
    torque_accum: &mut Torque
) {
    force_accum.add(force);
    torque_accum.add(point.cross(force));
}

/// Updates the force and torque accumulators based on a given force applied at a given point
/// in global coordinates.
fn add_force_at_point(
    centre_of_mass_position: DVec3,
    force: DVec3,
    mut point: DVec3,
    force_accum: &mut Force,
    torque_accum: &mut Torque
) {
    // convert point to body coords relative to centre of mass.
    point -= centre_of_mass_position;

    add_force_at_body_point(force, point, force_accum, torque_accum);
}
