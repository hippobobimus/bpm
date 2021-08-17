use bevy::math::DVec3;

#[derive(Debug, Default)]
/// A force accumulator, used to represent all forces currently applied to a body.
pub struct Force {
    total: DVec3,
}

impl Force {
    /// Creates a new force accumulator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds the given force to the accumulator.
    pub fn add(&mut self, f: DVec3) {
        self.total += f;
    }

    /// Resets the force accumulator to zero.
    pub fn reset(&mut self) {
        self.total = DVec3::ZERO;
    }

    /// Returns the current total force accumulated as a vector.
    pub fn vector(&self) -> &DVec3 {
        &self.total
    }
}
