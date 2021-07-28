use bevy::math::{DQuat, DVec3};

use crate::physics::prelude::*;

//use crate::constants;

#[derive(Debug)]
pub struct Rotator {
    axis: DVec3,
    positions: (DVec3, DVec3),
    forces: (DVec3, DVec3),
}

//impl Default for Rotator {
//    fn default() -> Self {
//        Self {
//            axis: DVec3::X,
//            position: DVec3::new(0.0, 0.0, 0.0),
//            other_position: DVec3::new(0.0, 0.0, 0.0),
//            force_magnitude: 0.0,
//        }
//    }
//}

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

//    pub fn force_and_body_point(&self) -> (DVec3, DVec3) {
//        let force = self.force_magnitude * (self.axis.cross(self.position));
//
//        println!("{} * {} x {} = {}", self.force_magnitude, self.axis, self.position, force);
//
//        (force, self.position)
//    }

    pub fn update_force(
        &self,
        force_accum: &mut Force,
        torque_accum: &mut Torque
    ) {
        use crate::physics::systems::forces;

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
