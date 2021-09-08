use bevy::prelude::*;

use crate::{
    constants,
    physics::components::{
        AngularVelocity,
        BoundaryCollider,
        Force,
        InertiaTensor,
        Mass,
        PhysTransform,
        Torque,
        Velocity,
    },
};

/// System labels covering sub-systems in the integration process.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum IntegratorSystems {
    Integrate,
    UpdateCache,
    ResetAccumulators,
}

/// A SystemSet that runs the integrator to update velocity (including angular velocity) and
/// transform components, subsequently updating any cached data that relies on the current
/// transform and resetting the accumulators.
pub fn get_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(integrate.system()
                     .label(IntegratorSystems::Integrate)
        )
        .with_system(update_cached_data.system()
                     .label(IntegratorSystems::UpdateCache)
                     .after(IntegratorSystems::Integrate)
        )
        .with_system(reset_accumulators.system()
                     .label(IntegratorSystems::ResetAccumulators)
                     .after(IntegratorSystems::Integrate)
        )
}

/// An integration system that updates Velocity/AngularVelocity and PhysTransform/Transform
/// components based on the attributes of the Entitys (Mass/InertiaTensor), the currently applied
/// Force and Torque, and the timestep.
fn integrate(
    time: Res<Time>,
    mut query: Query<(
        &mut AngularVelocity,
        &Force,
        &InertiaTensor,
        &Mass,
        &mut PhysTransform,
        &Torque,
        &mut Transform,
        &mut Velocity
    )>,
) {
    let dt_secs = time.delta_seconds_f64();

    for (mut ang_v, f, inertia_tensor, m, mut transform, torque,
         mut bevy_transform, mut v) in query.iter_mut()
    {
        // Infinite mass objects cannot move.
        if m.is_infinite() { continue };

        // Calculate linear acceleration from currently applied forces.
        let accel = f.vector() * m.inverse();

        // Calculate angular acceleration from torques.
        let ang_accel = inertia_tensor.inverse_global() * torque.vector();

        // Update linear velocity.
        v.add(accel * dt_secs);

        // Update angular velocity.
        ang_v.add(ang_accel * dt_secs);

        // Apply damping.
        v.scale(constants::DAMPING_FACTOR.powf(dt_secs));
        ang_v.scale(constants::ANGULAR_DAMPING_FACTOR.powf(dt_secs));

        // Update internal physics module rotation and translation.
        transform.rotation = (transform.rotation + ang_v.quaternion() * transform.rotation * dt_secs * 0.5)
            .normalize();
        transform.translation += v.vector() * dt_secs;

        // Update bevy 32 bit float transform based on physics module's 64 bit transform.
        bevy_transform.rotation = transform.rotation.as_f32();
        bevy_transform.translation = transform.translation.as_f32();

        // If velocity is very low, make it 0.
        if v.vector().length_squared() < constants::LOW_VELOCITY_THRESHOLD {
            v.zero();
        }
    }
}

/// Updates any cached derived data that relies on the PhysTransform, for Entitys that have moved.
fn update_cached_data(
    mut set: QuerySet<(
        Query<(&mut PhysTransform, &mut InertiaTensor), Changed<PhysTransform>>,
        Query<(&PhysTransform, &mut BoundaryCollider), Changed<PhysTransform>>,
    )>,
) {
    for (mut transform, mut inertia_tensor) in set.q0_mut().iter_mut() {
        transform.update();
        inertia_tensor.update(transform.matrix());
    }
    for (transform, mut boundary) in set.q1_mut().iter_mut() {
        // update the cached plane normal in the boundary if the boundary has moved.
        boundary.0.update(transform);
    }
}

/// A system that zeroes the Force and Torque accumulator components for all Entitys.
fn reset_accumulators(mut query: Query<(&mut Force, &mut Torque)>) {
    for (mut f, mut tq) in query.iter_mut() {
        f.reset();
        tq.reset();
    }
}
