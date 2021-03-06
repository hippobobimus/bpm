use bevy::{
    prelude::*,
};

use crate::physics::components::{
    Drag,
    Force,
    Gravity,
    Mass,
    PhysTransform,
    Rotator,
    Thrust,
    Torque,
    Velocity,
};

/// Force and torque system labels.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum ForceAndTorqueSystems {
    Accumulator,
    Reset,
}

/// A SystemSet that resets and then recalculates the forces and torques applied on a body.
pub fn get_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(reset_accumulators.system()
                     .label(ForceAndTorqueSystems::Reset)
        )
        .with_system(force_and_torque_accumulation.system()
                     .label(ForceAndTorqueSystems::Accumulator)
                     .after(ForceAndTorqueSystems::Reset)
        )
}

/// A system that calculates and accumulates various forces and associated torques applied on a
/// body.
fn force_and_torque_accumulation(
    mut q: QuerySet<(
        Query<(&mut Drag, &mut Force, &Velocity)>,
        Query<(&Gravity, &mut Force, &Mass)>,
        Query<(&Thrust, &mut Force)>,
        Query<(&Rotator, &mut Force, &mut Torque, &PhysTransform)>,
    )>,
) {
    // Apply force generators.
    for (mut drag, mut f, v) in q.q0_mut().iter_mut() {
        drag.update_force(&mut f, v.vector());
    }
    for (gravity, mut f, m) in q.q1_mut().iter_mut() {
        // ensure the mass is not 0 or infinite (or subnormal/NaN).
        if !m.is_normal() { break };
        gravity.update_force(&mut f, m.value());
    }
    for (thrust, mut f) in q.q2_mut().iter_mut() {
        thrust.update_force(&mut f);
    }

    // Apply force and torque generators.
    for (rotator, mut f, mut torque, transform) in q.q3_mut().iter_mut() {
        rotator.update_force_and_torque(&mut f, &mut torque, transform);
    }
}

/// A system that zeroes the Force and Torque accumulator components for all Entitys.
fn reset_accumulators(mut query: Query<(&mut Force, &mut Torque)>) {
    for (mut f, mut tq) in query.iter_mut() {
        f.reset();
        tq.reset();
    }
}
