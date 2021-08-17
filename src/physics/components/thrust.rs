use bevy::math::DVec3;

use crate::constants;

#[derive(Debug)]
/// A force generator representing thrust on a body.
pub struct Thrust {
    force: DVec3,
    magnitude: f64,
}

impl Thrust {
    /// Creates a new thrust force generator, where the thrust force(s) when applied will be of the
    /// given magnitude.
    pub fn new(magnitude: f64) -> Self {
        Self {
            force: Default::default(),
            magnitude,
        }
    }

    /// Cuts thrust in the given direction.
    pub fn disengage(&mut self, dir: &DVec3) {
        self.force -= self.magnitude * dir.normalize();
    }

    /// Adds thrust in the given direction.
    pub fn engage(&mut self, dir: &DVec3) {
        self.force += self.magnitude * dir.normalize();
    }

    /// Returns the force currently generated by thrust.
    pub fn force(&self) -> &DVec3 {
        &self.force
    }
}

impl Default for Thrust {
    fn default() -> Self {
        Self {
            force: Default::default(),
            magnitude: constants::DEFAULT_THRUST,
        }
    }
}
