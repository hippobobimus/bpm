use bevy::math::DVec3;

use crate::constants;

#[derive(Debug)]
pub struct Gravity {
    g: DVec3,
}

impl Default for Gravity {
    fn default() -> Self {
        Self {
            g: *constants::DEFAULT_GRAVITY,
        }
    }
}

impl Gravity {
    pub fn new(g: DVec3) -> Self {
        Self { g }
    }

    pub fn force(&self, m: f64) -> DVec3 {
        m * self.g
    }
}
