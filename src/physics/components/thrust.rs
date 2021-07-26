use bevy::math::DVec3;

use crate::constants;

pub struct Thrust {
    force: DVec3,
    magnitude: f64,
}

impl Default for Thrust {
    fn default() -> Self {
        Self {
            force: Default::default(),
            magnitude: constants::DEFAULT_THRUST,
        }
    }
}

impl Thrust {
    pub fn new(magnitude: f64) -> Self {
        Self {
            force: Default::default(),
            magnitude,
        }
    }

    pub fn disengage(&mut self, dir: &DVec3) {
        self.force -= self.magnitude * dir.normalize();
    }

    pub fn engage(&mut self, dir: &DVec3) {
        self.force += self.magnitude * dir.normalize();
    }

    pub fn force(&self) -> &DVec3 {
        &self.force
    }
}
