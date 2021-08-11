use bevy::{
    math::{DMat4, DQuat, DVec3},
};

/// Duplication of the built-in Bevy Transform component with a higher precision, for use
/// internally within the physics engine's calculations.
pub struct PhysTransform {
    pub rotation: DQuat,
    pub translation: DVec3,
    // cache transform matrix to save re-calculating unnecessarily
    matrix: DMat4,
}

impl PhysTransform {
    //TODO normalize rotation Quaternion?
    pub const IDENTITY: Self = Self {
        translation: DVec3::ZERO,
        rotation: DQuat::IDENTITY,
        matrix: DMat4::IDENTITY,
    };

    // Instantiation

    pub fn from_rotation_translation(rotation: DQuat, translation: DVec3) -> Self {
        let mut result = Self {
            rotation: rotation.normalize(),
            translation,
            ..Default::default()
        };
        result.compute_matrix();
        result
    }

    pub fn from_rotation(rotation: DQuat) -> Self {
        Self::from_rotation_translation(rotation, DVec3::ZERO)
    }

    pub fn from_translation(translation: DVec3) -> Self {
        Self::from_rotation_translation(DQuat::IDENTITY, translation)
    }

    pub fn from_xyz(x: f64, y: f64, z: f64) -> Self {
        Self::from_translation(DVec3::new(x, y, z))
    }

    // Getters

    /// Returns the transformed x, y or z axis corresponding to the given index in the
    /// tranform matrix. (0 = x, 1 = y, 2 = z).
    pub fn axis(&self, index: usize) -> DVec3 {
        self.matrix.col(index).truncate()
    }

    pub fn rotation(&self) -> DQuat {
        self.rotation
    }

    pub fn translation(&self) -> DVec3 {
        self.translation
    }

    pub fn matrix(&self) -> DMat4 {
        self.matrix
    }

    // Misc

    pub fn mul_vec3(&self, value: DVec3) -> DVec3 {
        value = self.rotation * value;
        value += self.translation;
        value
    }

    // Update

    // TODO keep this or move to expiry methodology?
    pub fn calc_derived_data(&mut self) {
        self.compute_matrix();
    }

    // Helpers

    fn compute_matrix(&mut self) {
        self.matrix = DMat4::from_rotation_translation(self.rotation, self.translation);
    }
}

impl Default for PhysTransform {
    fn default() -> Self {
        Self::IDENTITY
    }
}
