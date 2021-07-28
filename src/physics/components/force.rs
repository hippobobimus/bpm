use bevy::math::DVec3;

// -- Force Accumulator

#[derive(Debug, Default)]
pub struct Force {
    total: DVec3,
}

impl Force {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, f: DVec3) {
        self.total += f;
    }

    pub fn reset(&mut self) {
        self.total = DVec3::ZERO;
    }

    pub fn vector(&self) -> &DVec3 {
        &self.total
    }
}
