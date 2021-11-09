use bevy::math::DVec3;

use crate::{
    constants,
    physics::components::Force,
    physics::systems::force_and_torque,
};

#[derive(Debug)]
/// A force generator that represents drag on a body.
pub struct Drag {
    k1: f64,
    k2: f64,
    last: DVec3, // cached value.
}

impl Drag {
    /// Creates a new Drag component with the given drag coefficients. The 'k1' coefficient scales
    /// the drag proportional to the velocity, whilst the 'k2' coefficient scales the drag
    /// proportional to the square velocity.
    pub fn new(k1: f64, k2: f64) -> Self {
        Self { k1, k2, last: DVec3::ZERO }
    }

    /// Adds the force currently generated by drag, on a body with the given velocity, to the given
    /// Force accumulator.
    pub fn update_force(&mut self, force_accum: &mut Force, velocity: DVec3) {
        // Drag is considered to act through the centre of mass and so not introduce any torque.
        force_and_torque::add_force(self.force(velocity), force_accum);
    }

    /// Returns the force currently generated by drag.
    fn force(&mut self, velocity: DVec3) -> DVec3 {
        let v_mag = velocity.length();
        let coeff = self.k1 * v_mag + self.k2 * v_mag.powi(2);

        let drag = -coeff * velocity.normalize_or_zero();

        self.last = drag;

        drag
    }

    /// Returns the last calculated drag as a vector.
    pub fn vector(&self) -> DVec3 {
        self.last
    }
}

impl Default for Drag {
    fn default() -> Self {
        Self {
            k1: constants::DEFAULT_K1,
            k2: constants::DEFAULT_K2,
            last: DVec3::ZERO,
        }
    }
}
