use bevy::math::{DQuat, DVec3};

#[derive(Debug, Default)]
/// A component that describes the angular velocity of a body.
pub struct AngularVelocity {
    vector: DVec3,
}

impl AngularVelocity {
    /// Creates a new component with the given angular velocity.
    pub fn new(ang_vel: DVec3) -> Self {
        Self { vector: ang_vel }
    }

    /// Adds the given angular velocity.
    pub fn add(&mut self, v: DVec3) {
        self.vector += v;
    }

    /// Returns the angular velocity quaternion.
    ///
    /// Equivalent to:
    /// 
    /// (w, x, y, z) = (0, ang_vel[x], ang_vel[y], ang_vel[z])
    pub fn quaternion(&self) -> DQuat {
        DQuat::from_xyzw(
            self.vector.x,
            self.vector.y,
            self.vector.z,
            0.0,
        )
    }

    /// Scales the angular velocity by the given value.
    pub fn scale(&mut self, s: f64) {
        self.vector *= s;
    }

    /// Returns the vector form of the angular velocity.
    pub fn vector(&self) -> DVec3 {
        self.vector
    }

    /// Resets the angular velocity to zero.
    pub fn zero(&mut self) {
        self.vector = DVec3::ZERO;
    }
}
