use bevy::math::{DQuat, DVec3};

#[derive(Default)]
pub struct AngularVelocity {
    vector: DVec3,
}

impl AngularVelocity {
    pub fn new(vector: DVec3) -> Self {
        Self { vector }
    }

    pub fn add(&mut self, v: DVec3) {
        self.vector += v;
    }

    /// The angular velocity quaternion.
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

    pub fn scale(&mut self, s: f64) {
        self.vector *= s;
    }

    pub fn vector(&self) -> &DVec3 {
        &self.vector
    }

    pub fn zero(&mut self) {
        self.vector = DVec3::ZERO;
    }
}
