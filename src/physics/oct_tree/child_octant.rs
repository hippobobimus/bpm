use bevy::math::DVec3;
use lazy_static::lazy_static;

use std::collections::HashMap;

/// Each node has eight children that correspond to subdivision of the node's bounding box into
/// equal octants. This enum is used to lookup a child node based on a specific octant position.
///
/// Labels represent the positive or negative halfspace in each axis that an octant corresponds to,
/// given an origin at the centre of the subdivided space. e.g. the 0th octant is in the negative
/// halfspace of all three axes so has the label NegNegNeg.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChildOctant {
    NegNegNeg = 0,
    PosNegNeg = 1,
    NegPosNeg = 2,
    PosPosNeg = 3,
    NegNegPos = 4,
    PosNegPos = 5,
    NegPosPos = 6,
    PosPosPos = 7,
}

impl ChildOctant {
    // All possible values of the enum.
    pub const VALUES: [Self; 8] = [
        Self::NegNegNeg,
        Self::PosNegNeg,
        Self::NegPosNeg,
        Self::PosPosNeg,
        Self::NegNegPos,
        Self::PosNegPos,
        Self::NegPosPos,
        Self::PosPosPos,
    ];

    /// Returns a vector describing the offset direction of a child node's centre from its
    /// parent's centre. The vector has either value 1 or -1 in each axis.
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
            ChildOctant::NegNegNeg,
            DVec3::new(-1.0, -1.0, -1.0),
        );
        map.insert(
            ChildOctant::PosNegNeg,
            DVec3::new(1.0, -1.0, -1.0),
        );
        map.insert(
            ChildOctant::NegPosNeg,
            DVec3::new(-1.0, 1.0, -1.0),
        );
        map.insert(
            ChildOctant::PosPosNeg,
            DVec3::new(1.0, 1.0, -1.0),
        );
        map.insert(
            ChildOctant::NegNegPos,
            DVec3::new(-1.0, -1.0, 1.0),
        );
        map.insert(
            ChildOctant::PosNegPos,
            DVec3::new(1.0, -1.0, 1.0),
        );
        map.insert(
            ChildOctant::NegPosPos,
            DVec3::new(-1.0, 1.0, 1.0),
        );
        map.insert(
            ChildOctant::PosPosPos,
            DVec3::new(1.0, 1.0, 1.0),
        );
        map
    };
}
