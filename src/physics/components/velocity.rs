use bevy::math::DVec3;

#[derive(Default)]
/// A component representing the velocity of a body.
pub struct Velocity {
    vector: DVec3,
}

impl Velocity {
    /// Creates a new Velocity component with the given value.
    pub fn new(vector: DVec3) -> Self {
        Self { vector }
    }

    /// Adds the given velocity vector.
    pub fn add(&mut self, v: DVec3) {
        self.vector += v;
    }

    /// Multiplies the velocity vector by the given scaling factor.
    pub fn scale(&mut self, s: f64) {
        self.vector *= s;
    }

    /// Returns the vector representation of the velocity.
    pub fn vector(&self) -> DVec3 {
        self.vector
    }

    /// Sets the velocity to zero.
    pub fn zero(&mut self) {
        self.vector = DVec3::ZERO;
    }
}
