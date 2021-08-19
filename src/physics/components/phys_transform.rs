use bevy::{
    math::{DMat4, DQuat, DVec3},
};

/// Duplication of the built-in Bevy Transform component with a higher float precision (64 bit),
/// for use internally within the physics engine's calculations.
#[derive(Debug)]
pub struct PhysTransform {
    pub rotation: DQuat,
    pub translation: DVec3,
    // cache transform matrix to save re-calculating unnecessarily
    matrix: DMat4,
    inverse_matrix: DMat4,
}

impl PhysTransform {
    pub const IDENTITY: Self = Self {
        translation: DVec3::ZERO,
        rotation: DQuat::IDENTITY,
        matrix: DMat4::IDENTITY,
        inverse_matrix: DMat4::IDENTITY,
    };

    /// Creates the transform from the given rotation and translation.
    pub fn from_rotation_translation(rotation: DQuat, translation: DVec3) -> Self {
        let mut result = Self {
            rotation: rotation.normalize(),
            translation,
            ..Default::default()
        };
        // calculate the derived data.
        result.update();
        result
    }

    /// Creates the transform from the given rotation.
    pub fn from_rotation(rotation: DQuat) -> Self {
        Self::from_rotation_translation(rotation, DVec3::ZERO)
    }

    /// Creates the transform from the given translation.
    pub fn from_translation(translation: DVec3) -> Self {
        Self::from_rotation_translation(DQuat::IDENTITY, translation)
    }

    /// Creates the transform from the given translation in x, y and z coordinates.
    pub fn from_xyz(x: f64, y: f64, z: f64) -> Self {
        Self::from_translation(DVec3::new(x, y, z))
    }

    /// Returns the transformed x, y or z axis corresponding to the given index in the
    /// tranform matrix. (0 = x, 1 = y, 2 = z).
    pub fn axis(&self, index: usize) -> DVec3 {
        self.matrix.col(index).truncate()
    }

    /// Returns the rotation as a quaternion.
    pub fn rotation(&self) -> DQuat {
        self.rotation
    }

    /// Returns the translation vector.
    pub fn translation(&self) -> DVec3 {
        self.translation
    }

    /// Returns the transform matrix. The 'update' function MUST be called beforehand if the
    /// rotation or translation has changed, otherwise the matrix will be out of date.
    pub fn matrix(&self) -> DMat4 {
        self.matrix
    }

    /// Transforms the given point into local space using the inverse transformation matrix.
    pub fn get_point_in_local_space(&self, point: DVec3) -> DVec3 {
        self.inverse_matrix.transform_point3(point)
    }

    /// Transforms the given point into global space using the transformation matrix.
    pub fn get_point_in_global_space(&self, point: DVec3) -> DVec3 {
        self.matrix.transform_point3(point)
    }

    /// Transforms the given direction into local space using the inverse transformation matrix as
    /// a rotation only.
    pub fn get_direction_in_local_space(&self, direction: DVec3) -> DVec3 {
        self.inverse_matrix.transform_vector3(direction)
    }

    /// Transforms the given direction into global space using the transformation matrix as a
    /// rotation only.
    pub fn get_direction_in_global_space(&self, direction: DVec3) -> DVec3 {
        self.matrix.transform_vector3(direction)
    }

    /// Updates the cached transform matrix based on the current rotation and translation.
    pub fn update(&mut self) {
        self.matrix = DMat4::from_rotation_translation(self.rotation, self.translation);
        self.inverse_matrix = self.matrix.inverse();
    }
}

impl Default for PhysTransform {
    fn default() -> Self {
        Self::IDENTITY
    }
}
