use bevy::math::DVec3;

/// A torque accumulator.
#[derive(Debug, Default)]
pub struct Torque {
    total: DVec3,
}

impl Torque {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, torque: DVec3) {
        self.total += torque;
    }

    pub fn reset(&mut self) {
        self.total = DVec3::ZERO;
    }

    pub fn vector(&self) -> &DVec3 {
        &self.total
    }
}
