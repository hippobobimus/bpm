use bevy::math::DVec3;

#[derive(Default)]
pub struct Velocity {
    vector: DVec3,
}

impl Velocity {
    pub fn new(vector: DVec3) -> Self {
        Self { vector }
    }

    pub fn scale(&mut self, s: f64) {
        self.vector *= s;
    }

    pub fn add(&mut self, v: DVec3) {
        self.vector += v;
    }

    pub fn vector(&self) -> &DVec3 {
        &self.vector
    }

    pub fn zero(&mut self) {
        self.vector = DVec3::ZERO;
    }
}
