use bevy::{
    math::{DMat4, DQuat, DVec3},
};

/// Duplication of the built-in Bevy Transform component with a higher precision, for use
/// internally within the physics engine's calculations.
pub struct PhysTransform {
    rotation: DQuat,
    translation: DVec3,
    // cache transform matrix to save re-calculating unnecessarily
    matrix: DMat4,
}

impl PhysTransform {
    pub fn from_xyz(x: f64, y: f64, z: f64) -> Self {
        Self::from_translation(DVec3::new(x, y, z))
    }

    pub fn from_translation(translation: DVec3) -> Self {
        Self {
            translation,
            ..Default::default()
        }
    }

    pub fn identity() -> Self {
        Self {
            translation: DVec3::ZERO,
            rotation: DQuat::IDENTITY,
            matrix: DMat4::IDENTITY,
        }
    }

    pub fn rotation(&self) -> &DQuat {
        &self.rotation
    }

    pub fn translation(&self) -> &DVec3 {
        &self.translation
    }

    pub fn matrix(&self) -> &DMat4 {
        &self.matrix
    }

    pub fn calc_derived_data(&mut self) {
        self.compute_matrix();
    }

    fn compute_matrix(&mut self) {
        self.matrix = DMat4::from_rotation_translation(self.rotation, self.translation);
    }
}

impl Default for PhysTransform {
    fn default() -> Self {
        Self::identity()
    }
}
