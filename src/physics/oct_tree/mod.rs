pub mod child_octant;
pub mod node;
pub mod oct_tree;

/// The unique positional index of a node in the OctTree's arena. Used to retrieve nodes.
pub type OctIndex = usize;

pub use crate::physics::oct_tree::{
    child_octant::ChildOctant,
    oct_tree::OctTree,
};
