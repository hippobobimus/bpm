use bevy::math::DVec3;
use lazy_static::lazy_static;

use std::collections::HashMap;

/// Each node has eight children that correspond to subdivision of the node's bounding box into
/// equal octants. This enum is used to lookup a child node based on a specific octant position.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChildOctant {
    BottomBackLeft = 0,
    BottomBackRight = 1,
    BottomFrontLeft = 2,
    BottomFrontRight = 3,
    TopBackLeft = 4,
    TopBackRight = 5,
    TopFrontLeft = 6,
    TopFrontRight = 7,
}

impl ChildOctant {
    // All possible values of the enum.
    const VALUES: [Self; 8] = [
        Self::BottomBackLeft,
        Self::BottomBackRight,
        Self::BottomFrontLeft,
        Self::BottomFrontRight,
        Self::TopBackLeft,
        Self::TopBackRight,
        Self::TopFrontLeft,
        Self::TopFrontRight,
    ];

    /// Returns a unit vector describing the offset direction of a child node's centre from its
    /// parent's centre.
    pub fn centre_offset(&self) -> DVec3 {
        *OCTANT_DIRECTION_MAP.get(self).expect("OCTANT_DIRECTION_MAP is incomplete!")
    }
}

// A mapping that associates a box's octants to the direction of their respective centres from the
// centre of the box.
lazy_static! {
    static ref OCTANT_DIRECTION_MAP: HashMap<ChildOctant, DVec3> = {
        let mut map = HashMap::new();
        map.insert(
            ChildOctant::BottomBackLeft,
            DVec3::new(-1.0, -1.0, -1.0).normalize(),
        );
        map.insert(
            ChildOctant::BottomBackRight,
            DVec3::new(1.0, -1.0, -1.0).normalize(),
        );
        map.insert(
            ChildOctant::BottomFrontLeft,
            DVec3::new(-1.0, -1.0, 1.0).normalize(),
        );
        map.insert(
            ChildOctant::BottomFrontRight,
            DVec3::new(1.0, -1.0, 1.0).normalize(),
        );
        map.insert(
            ChildOctant::TopBackLeft,
            DVec3::new(-1.0, 1.0, -1.0).normalize(),
        );
        map.insert(
            ChildOctant::TopBackRight,
            DVec3::new(1.0, 1.0, -1.0).normalize(),
        );
        map.insert(
            ChildOctant::TopFrontLeft,
            DVec3::new(-1.0, 1.0, 1.0).normalize(),
        );
        map.insert(
            ChildOctant::TopFrontRight,
            DVec3::new(1.0, 1.0, 1.0).normalize(),
        );
        map
    };
}
