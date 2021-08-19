use bevy::math::DVec3;

/// A torque accumulator.
#[derive(Debug, Default)]
pub struct Torque {
    total: DVec3,
}

impl Torque {
    /// Creates a new empty torque accumulator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds the given torque to the accumulator.
    pub fn add(&mut self, torque: DVec3) {
        self.total += torque;
    }

    /// Resets the accumulator to zero.
    pub fn reset(&mut self) {
        self.total = DVec3::ZERO;
    }

    /// Returns the total torque accumulated as a vector.
    pub fn vector(&self) -> DVec3 {
        self.total
    }
}
