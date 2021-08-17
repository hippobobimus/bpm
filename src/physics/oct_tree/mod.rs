mod child_octant;
mod node;
mod oct_tree;

/// The unique positional index of a node in the OctTree's arena. Used to retrieve nodes.
pub type OctIndex = usize;

// Re-exports
pub use child_octant::ChildOctant;
pub use oct_tree::OctTree;
pub use node::OctTreeNode;
