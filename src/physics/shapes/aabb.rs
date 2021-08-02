use bevy::math::DVec3;

use crate::physics::shapes::*;

/// An axis-aligned 3D bounding box. It stores the half-width extents of the bounding box in the x,
/// y and z directions, but does not directly store the box's position within the coordinate system.
#[derive(Debug, Copy, Clone)]
pub struct Aabb3D {
    extents: DVec3, // half-width extents in x, y and z axes.
}

impl Aabb3D {
    /// Creates a new axis-aligned bounding box with the given extents (half-widths) in the x, y
    /// and z directions.
    pub fn new(extent_x: f64, extent_y: f64, extent_z: f64) -> Self {
        Self {
            extents: DVec3::new(extent_x, extent_y, extent_z),
        }
    }

    /// Returns the extents (half widths) of the aabb as an x, y, z vector.
    pub fn extents(&self) -> &DVec3 {
        &self.extents
    }

    /// Calculates and returns the minimum vertex of the aabb in global coords, given its current
    /// centre position in global coords.
    pub fn min(&self, centre: DVec3) -> DVec3 {
        centre - self.extents
    }

    /// Calculates and returns the maximum vertex of the aabb in global coords, given its current
    /// centre position in global coords.
    pub fn max(&self, centre: DVec3) -> DVec3 {
        centre + self.extents
    }

    /// Returns true if the given sphere is wholly enclosed within the box.
    pub fn holds_sphere(
        &self, sphere: Sphere, sphere_position: DVec3, aabb_position: DVec3
    ) -> bool {
        let dmin = sphere_position - self.min(aabb_position);
        let dmax = self.max(aabb_position) - sphere_position;
        let r = sphere.radius();

        if dmin.x < r || dmin.y < r || dmin.z < r {
            return false;
        }
        if dmax.x < r || dmax.y < r || dmax.z < r {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_holds_sphere() {
        let aabb = Aabb3D::new(2.0, 2.0, 2.0);
        let sphere = Sphere::new(1.0);

        let aabb_pos = DVec3::ZERO;

        // check sphere well inside box.
        let mut sphere_pos = DVec3::ZERO;
        assert!(aabb.holds_sphere(sphere, sphere_pos, aabb_pos));

        // check sphere up to but not over boundary in each axis individually.
        for i in [DVec3::X, DVec3::Y, DVec3::Z].iter() {
            sphere_pos = DVec3::new(0.0, 0.0, 0.0) + *i;
            assert!(aabb.holds_sphere(sphere, sphere_pos, aabb_pos));
            sphere_pos = DVec3::new(0.0, 0.0, 0.0) - *i;
            assert!(aabb.holds_sphere(sphere, sphere_pos, aabb_pos));
        }

        // check overlap in each axis individually.
        for i in [DVec3::X, DVec3::Y, DVec3::Z].iter() {
            sphere_pos = DVec3::new(0.0, 0.0, 0.0) + *i * 1.001;
            assert!(!aabb.holds_sphere(sphere, sphere_pos, aabb_pos));
            sphere_pos = DVec3::new(0.0, 0.0, 0.0) - *i * 1.001;
            assert!(!aabb.holds_sphere(sphere, sphere_pos, aabb_pos));
        }

        // check sphere way outside box in each axis individually.
        for i in [DVec3::X, DVec3::Y, DVec3::Z].iter() {
            sphere_pos = DVec3::new(0.0, 0.0, 0.0) + *i * 10.0;
            assert!(!aabb.holds_sphere(sphere, sphere_pos, aabb_pos));
            sphere_pos = DVec3::new(0.0, 0.0, 0.0) - *i * 10.0;
            assert!(!aabb.holds_sphere(sphere, sphere_pos, aabb_pos));
        }
    }
}
