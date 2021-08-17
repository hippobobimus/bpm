use bevy::prelude::*;

use crate::{
    constants,
    physics::components::{
        AngularVelocity,
        Force,
        InertiaTensor,
        Mass,
        PhysTransform,
        Torque,
        Velocity,
    },
};

pub fn integrator(
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
         mut bevy_transform, mut v) in query.iter_mut() {

        // Infinite mass objects cannot move.
        if m.is_infinite() { break };

        // Calculate linear acceleration from currently applied forces.
        let accel = *f.vector() * m.inverse();

        // Calculate angular acceleration from torques.
        let ang_accel = *inertia_tensor.inverse() * *torque.vector();

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
        transform.translation += *v.vector() * dt_secs;

        // Update bevy transform based on physics module's transform.
        bevy_transform.rotation = transform.rotation.as_f32();
        bevy_transform.translation = transform.translation.as_f32();

        // If velocity is very low, make it 0
        if v.vector().length_squared() < constants::LOW_VELOCITY_THRESHOLD {
            v.zero();
        }

        // TODO reset forces here?
    }
}
