use bevy::math::DVec3;

use crate::physics::shapes::Collider;

/// A 6-sided polygon described by its extents in local body coords. The cuboid is axis aligned in
/// local body space with the origin at the centre of the cuboid.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cuboid {
    extents: DVec3,
}

impl Cuboid {
    /// Creates a new cuboid with the given extents relative to the local body coords.
    pub fn new(extents: DVec3) -> Self {
        Self { extents }
    }

    /// Returns the extents of the cuboid in local body space.
    pub fn extents(&self) -> DVec3 {
        self.extents
    }
}

impl Collider for Cuboid {}
