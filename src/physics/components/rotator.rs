use bevy::math::{
    DQuat,
    DVec3
};

use crate::{
    physics::components::{
        Force,
        Torque,
    },
    physics::systems::forces,
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
    /// to create a rotation around the given axis.
    pub fn new(axis: DVec3, position: DVec3, force_magnitude: f64) -> Self {
        let rotation = DQuat::from_axis_angle(axis, std::f64::consts::PI);

        let force = force_magnitude * (axis.cross(position).normalize());

        Self {
            axis: axis.normalize(),
            positions: (position, rotation.mul_vec3(position)),
            forces: (force, rotation.mul_vec3(force)),
        }
    }

    /// Updates the force and torque accumulators based on the current rotator forces.
    pub fn update_force(
        &self,
        force_accum: &mut Force,
        torque_accum: &mut Torque
    ) {
        forces::add_force_at_body_point(
            self.forces.0,
            self.positions.0,
            force_accum,
            torque_accum,
        );
        forces::add_force_at_body_point(
            self.forces.1,
            self.positions.1,
            force_accum,
            torque_accum,
        );
    }
}
