use bevy::math::DVec3;

use std::collections::HashSet;

use crate::{
    physics::oct_tree::{ChildOctant, OctIndex},
    physics::shapes::*,
};

/// Each node has a unique index that locates it within the oct-tree arena, an outer boundary as a
/// centre position and axis-aligned bounding box, a set of stored data and an array of 8 optional
/// child node indices.
#[derive(Debug)]
pub struct OctTreeNode<T: Copy> {
    idx: OctIndex,
    pub centre: DVec3,
    pub boundary: Aabb3D,
    pub data: HashSet<T>,
    pub children: [Option<OctIndex>; 8],
}

impl<T: Copy> OctTreeNode<T> {
    /// Creates an empty node with the given index, center position and outer boundary.
    pub fn new(idx: OctIndex, centre: DVec3, boundary: Aabb3D) -> Self {
        Self {
            idx,
            centre,
            boundary,
            data: HashSet::new(),
            children: Default::default(),
        }
    }

    /// Returns the index of this node.
    pub fn get_idx(&self) -> OctIndex {
        self.idx
    }

    /// Returns the set of data stored in this node.
    pub fn get_data(&self) -> &HashSet<T> {
        &self.data
    }

    /// Returns an array of options containing indices of child nodes connected to this node.
    pub fn get_child_indices(&self) -> [Option<OctIndex>; 8] {
        self.children
    }

    /// Returns an option containing the index of the child node requested by its octant position,
    /// or None if there is no connection.
    pub fn get_child_idx(&self, c: ChildOctant) -> Option<OctIndex> {
        self.children[c as usize]
    }

    /// Creates a connection from the node to a child node via the given octant.
    pub fn set_child_idx(&mut self, octant: ChildOctant, child_node_idx: OctIndex) {
        self.children[octant as usize] = Some(child_node_idx);
    }
}

