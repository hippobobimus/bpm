use bevy::math::{DMat3, DMat4, DVec3};

use crate::constants;

pub struct InertiaTensor {
    tensor: DMat3,
    inverse: DMat3,
    // cache rather than recalculate unnecessarily
    inverse_global: DMat3,
}

impl Default for InertiaTensor {
    fn default() -> Self {
        Self::sphere(constants::DEFAULT_MASS, constants::DEFAULT_RADIUS)
    }
}

impl InertiaTensor {
    pub fn new(inertia_tensor: DMat3) -> Self {
        Self {
            tensor: inertia_tensor,
            inverse: inertia_tensor.inverse(),
            inverse_global: DMat3::ZERO,
        }
    }

    /// Instantiates an inertia tensor for a solid sphere with the given mass and radius.
    pub fn sphere(mass: f64, radius: f64) -> Self {
        let element = 0.4 * mass * radius.powi(2);

        Self::new(DMat3::from_diagonal(DVec3::new(element, element, element)))
    }

    /// Instantiates an inertia tensor for a cuboid (any 6-sided rectangular object with constant
    /// density) with the given mass and extents in the x, y and z axes.
    pub fn cuboid(mass: f64, dx: f64, dy: f64, dz: f64) -> Self {
        let coeff = mass / 12.0;

        Self::new(DMat3::from_diagonal(DVec3::new(
                    coeff * (dy.powi(2) + dz.powi(2)),
                    coeff * (dx.powi(2) + dz.powi(2)),
                    coeff * (dx.powi(2) + dy.powi(2)),
        )))
    }

    // TODO inertia tensors for other standard shapes.
    // Cuboid, ellipsoid, shell-sphere, cylinder, cone, hemisphere...

    pub fn tensor(&self) -> &DMat3 {
        &self.tensor
    }

    pub fn inverse(&self) -> &DMat3 {
        &self.inverse
    }

    /// The inverse inertia tensor in global coordinates.
    pub fn inverse_global(&self) -> &DMat3 {
        &self.inverse_global
    }

    /// Transforms the inverse inertia tensor into a new basis using the given transform matrix and
    /// stores it in the inverse_global field.
    fn transform_inertia_tensor(&mut self, transform_matrix: &DMat4) {
        // TODO will be able to simplify to the below when bevy uses a more recent version of the
        // glam crate.
        //let tm3 = DMat3::from_mat4(transform_matrix);

        // discard 3rd column and 3rd row.
        let tm3 = DMat3::from_cols(
            transform_matrix.col(0).truncate(),
            transform_matrix.col(1).truncate(),
            transform_matrix.col(2).truncate()
        );

        self.inverse_global = tm3 * self.inverse * tm3.inverse();
    }

    /// Updates cached values. Must be run when the associated entity is translated or rotated
    /// within global coordinate space to ensure an accurate value for the inverse inertia tensor
    /// in global coords.
    pub fn update(&mut self, transform_matrix: &DMat4) {
        self.transform_inertia_tensor(transform_matrix);
    }
}
