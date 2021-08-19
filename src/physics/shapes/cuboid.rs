use bevy::math::DVec3;

use crate::{
    physics::shapes::{
        Collidable,
        CollisionPrimative, 
        Sphere,
    },
    physics::components::PhysTransform,
};

/// A 6-sided polygon described by its extents in local body coords. The cuboid is axis aligned in
/// local body space with the origin at the centre of the cuboid.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cuboid {
    extents: DVec3,
    bounding_sphere: Sphere,
}

impl Cuboid {
    /// Creates a new cuboid with the given extents relative to the local body coords.
    pub fn new(extents: DVec3) -> Self {
        Self {
            extents,
            bounding_sphere: Sphere::new(extents.length()),
        }
    }

    /// Returns the extents of the cuboid in local body space.
    pub fn extents(&self) -> DVec3 {
        self.extents
    }

    /// Returns a list of the cuboid's 8 vertices in global coords.
    pub fn vertices(&self, transform: &PhysTransform) -> [DVec3; 8] {
        let mut vertices = [
            DVec3::new(self.extents.x, self.extents.y, self.extents.z),
            DVec3::new(self.extents.x, self.extents.y, -self.extents.z),
            DVec3::new(self.extents.x, -self.extents.y, self.extents.z),
            DVec3::new(self.extents.x, -self.extents.y, -self.extents.z),
            DVec3::new(-self.extents.x, self.extents.y, self.extents.z),
            DVec3::new(-self.extents.x, self.extents.y, -self.extents.z),
            DVec3::new(-self.extents.x, -self.extents.y, self.extents.z),
            DVec3::new(-self.extents.x, -self.extents.y, -self.extents.z)
        ];

        // Convert to global coords.
        for v in &mut vertices {
            *v = transform.get_point_in_global_space(*v);
        }

        vertices
    }

    /// Projects the half-size of the box with the given transform onto the given axis.
    pub fn project_onto_axis(&self, transform: &PhysTransform, axis: DVec3) -> f64 {
        self.extents.x * (axis.dot(transform.axis(0))).abs() +
            self.extents.y * (axis.dot(transform.axis(1))).abs() +
            self.extents.z * (axis.dot(transform.axis(2))).abs()
    }
}

impl CollisionPrimative for Cuboid {
    /// Returns the Sphere that shares a centre point with the Cuboid and completely encloses it.
    fn bounding_sphere(&self) -> &Sphere {
        &self.bounding_sphere
    }
}

impl Collidable for Cuboid {
    /// Calculates and returns the closest point on the Cuboid, with the given transform, to the
    /// given target point.
    fn closest_point_to(&self, transform: &PhysTransform, target: DVec3) -> DVec3 {
        // The calculation is made by clamping the target to the min and max vertices of the Cuboid.

        // Convert the target into local coords.
        let target_local = transform.get_point_in_local_space(target);
        let min = -self.extents;
        let max = self.extents;

        let result_local = target_local.clamp(min, max);

        // Convert back to global coords.
        transform.get_point_in_global_space(result_local)
    }

    /// Calculates and returns the shortest distance between the Cuboid, with the given transform,
    /// and the target point in global coords.
    fn shortest_distance_to(&self, transform: &PhysTransform, target: DVec3) -> f64 {
        (target - self.closest_point_to(transform, target)).length()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vertices_list() {
        let c = Cuboid::new(DVec3::new(2.0, 3.0, 4.0));

        let expected = [
            DVec3::new(2.0, 3.0, 4.0),
            DVec3::new(2.0, 3.0, -4.0),
            DVec3::new(2.0, -3.0, 4.0),
            DVec3::new(2.0, -3.0, -4.0),
            DVec3::new(-2.0, 3.0, 4.0),
            DVec3::new(-2.0, 3.0, -4.0),
            DVec3::new(-2.0, -3.0, 4.0),
            DVec3::new(-2.0, -3.0, -4.0)
        ];

        assert_eq!(expected, c.vertices(&PhysTransform::IDENTITY));
    }
}
