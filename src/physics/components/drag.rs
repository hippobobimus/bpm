use bevy::math::DVec3;

use crate::constants;

#[derive(Debug)]
pub struct Drag {
    k1: f64,
    k2: f64,
}

impl Default for Drag {
    fn default() -> Self {
        Self {
            k1: constants::DEFAULT_K1,
            k2: constants::DEFAULT_K2,
        }
    }
}

impl Drag {
    pub fn new(k1: f64, k2: f64) -> Self {
        Self { k1, k2 }
    }

    pub fn force(&self, velocity: DVec3) -> DVec3 {
        let v_mag = velocity.length();
        let coeff = self.k1 * v_mag + self.k2 * v_mag.powi(2);

        -coeff * velocity.normalize_or_zero()
    }
}
