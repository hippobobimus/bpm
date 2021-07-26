use bevy::math::DVec3;

#[derive(Default)]
pub struct AngularVelocity {
    vector: DVec3,
}

impl AngularVelocity {
    pub fn new(vector: DVec3) -> Self {
        Self { vector }
    }

    pub fn vector(&self) -> &DVec3 {
        &self.vector
    }
}
