use bevy::math::{
    DQuat,
    DVec3
};

use crate::{
    physics::components::{
        Force,
        PhysTransform,
        Torque,
    },
    physics::systems::force_and_torque,
};

#[derive(Debug)]
/// A force and torque generator representing rotation about an axis.
pub struct Rotator {
    axis: DVec3,
    positions: (DVec3, DVec3),
    forces: (DVec3, DVec3),
}

impl Rotator {
    /// Creates a new Rotator from the force with given magnitude that acts at the given position
    /// to create a rotation around the given axis. The position and axis are in body coords.
    pub fn new(axis: DVec3, position: DVec3, force_magnitude: f64) -> Self {
        // 180 deg. rotation about the given axis.
        let rotation = DQuat::from_axis_angle(axis, std::f64::consts::PI);

        let force = force_magnitude * (axis.cross(position).normalize());

        Self {
            axis: axis.normalize(),
            positions: (position, rotation.mul_vec3(position)),
            forces: (force, rotation.mul_vec3(force)),
        }
    }

    /// Updates the force and torque accumulators based on the current rotator forces.
    pub fn update_force_and_torque(
        &self,
        force_accum: &mut Force,
        torque_accum: &mut Torque,
        transform: &PhysTransform,
    ) {
        force_and_torque::add_body_force_at_body_point(
            self.forces.0,
            self.positions.0,
            transform,
            force_accum,
            torque_accum,
        );
        force_and_torque::add_body_force_at_body_point(
            self.forces.1,
            self.positions.1,
            transform,
            force_accum,
            torque_accum,
        );
    }
}
