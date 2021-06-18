use nalgebra::{
    vector,
};
use specs::{
    world::Index,
};

use crate::{
    bounding_shapes::{AABB, Circle},
};

pub type QTIndex = usize;

/// Each node has a unique index that locates it within the quadtree arena, an outer boundary as an
/// axis-aligned bounding box, a vector list of entity indices that correspond to entities in the
/// specs ECS and an array of 4 optional child node indices.
#[derive(Debug)]
struct Node {
    idx: usize,
    boundary: AABB,
    entities: Vec<Index>,
    children: [Option<usize>; 4],
}

impl Node {
    /// Creates an empty node with the given index and outer boundary.
    fn new(idx: usize, boundary: AABB) -> Self {
        Self {
            idx,
            boundary,
            entities: vec![],
            children: Default::default(),
        }
    }
}

/// An arena based QuadTree implementation that stores nodes in a vector.
#[derive(Debug, Default)]
pub struct QuadTree {
    arena: Vec<Node>,
    root: QTIndex,
}

impl QuadTree {
    const MAX_DEPTH: usize = 5;

    /// Creates a new QuadTree containing a single empty root node with the given boundary.
    pub fn new(boundary: AABB) -> Self {
        Self {
            arena: vec![Node::new(0, boundary)],
            root: 0,
        }
    }

    /// Creates a new node with the given outer boundary and adds it to the arena. The node's index
    /// in the arena is returned.
    fn add_node(&mut self, boundary: AABB) -> QTIndex {
        let idx = self.arena.len();

        let node = Node::new(idx, boundary);

        self.arena.push(node);

        idx
    }

    /// Retrieves a reference to the node located at the given arena index.
    fn get_node(&self, idx: QTIndex) -> Option<&Node> {
        self.arena.get(idx)
    }

    /// Calculates the child branch of a node that a given shape fits in.
    ///
    /// The given node's outer bounding box is split into 4 equal quadrants and the shape's
    /// position and dimensions are used to determine which of this (if any) it wholly fits within.
    ///
    /// The quadrants are indexed as follows, assuming an x-y coord system with an origin at the
    /// top left, x-axis that increases positively left->right and y-axis up->down:
    ///
    /// Top left: 0
    /// Top right: 1
    /// Bottom left: 2
    /// Bottom right 3
    ///
    /// If the shape does not fit wholly within any quadrant, None is returned.
    fn get_branch_idx(&self, node_idx: QTIndex, shape: Circle) -> Option<usize> {
        let mut idx = 0;

        let node = &self.arena[node_idx];

        // delta vector from object centre to bounding box centre.
        let boundary_centre = node.boundary.min + 0.5 * (node.boundary.max
                                                                 - node.boundary.min);
        let delta = shape.centre - boundary_centre;

        if (delta.x.abs() <= shape.radius) || (delta.y.abs() <= shape.radius) {
            return None;  // straddles multiple child nodes, or max depth reached.
        }

        for (i, d) in delta.iter().enumerate() {
            if *d > 0.0 { idx |= 1 << i }
        }
        
        Some(idx)
    }

    /// Inserts the given entity index into the tree according to its associated shape.
    // TODO return error if shape outside the bounds of the tree.
    pub fn insert(&mut self, shape: Circle, entity_idx: Index) {
        // traverse down branch, creating child nodes as required until the 
        // appropriate location to insert the entity has been found
        let mut node_idx = self.root;  // start at root
        let mut depth = 0;
        while let Some(branch_idx) = self.get_branch_idx(node_idx, shape) {

            if depth >= Self::MAX_DEPTH { break; }

            let node = &self.arena[node_idx];

            match node.children[branch_idx] {
                // child exists
                Some(child_idx) => {
                    // continue down the branch
                    node_idx = child_idx;
                }
                // else create child
                None => {
                    // Half-diagonal of bounding box.
                    let span = (node.boundary.max - node.boundary.min) * 0.5;

                    let child_min = match branch_idx {
                        0 => node.boundary.min,
                        1 => node.boundary.min + vector![span.x, 0.0],
                        2 => node.boundary.min + vector![0.0, span.y],
                        3 => node.boundary.min + span,
                        _ => panic!("index out of range"),
                    };

                    let child_max = child_min + span;

                    let child_boundary = AABB { min: child_min, max: child_max };

                    let child_idx = self.add_node(child_boundary);
                    self.arena[node_idx].children[branch_idx] = Some(child_idx);

                    // continue down the branch
                    node_idx = child_idx;
                }
            }
            depth += 1;
        }

        // finally insert the entity identifier at the location traversed to.
        self.arena[node_idx].entities.push(entity_idx);
    }

    /// Returns a custom iterator that performs a preorder traversal of the tree.
    pub fn preorder_iter(&self) -> PreorderIter {
        PreorderIter::new(self.root)
    }
}

/// A custom iterator that performs a preorder traversal of the tree.
pub struct PreorderIter {
    stack: Vec<QTIndex>,
}

impl PreorderIter {
    /// Returns a new iterator. Requires the root index.
    pub fn new(root: QTIndex) -> Self {
        Self {
            stack: vec![root],
        }
    }

    /// Returns the next node index in the preorder traversal of the tree. Takes a reference to the
    /// tree each time it is called to allow tree mutations between calls.
    pub fn next(&mut self, quad_tree: &QuadTree) -> Option<QTIndex> {
        if let Some(node_idx) = self.stack.pop() {
            if let Some(node) = quad_tree.get_node(node_idx) {
                // push the child node idx on to the stack in reverse order so that they are
                // served back in ascending quadrant order.
                for child in node.children.iter().rev() {
                    if let Some(child_idx) = child {
                        self.stack.push(*child_idx);
                    }
                }
            }
            return Some(node_idx);
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_branch_indexing() {
        let bounding_box = AABB {
            min: vector![0.0, 0.0],
            max: vector![100.0, 100.0],
        };

        let qt_1 = QuadTree::new(bounding_box);

        let circle_1 = Circle { centre: vector![10.0, 10.0], radius: 1.0 };
        let circle_2 = Circle { centre: vector![70.0, 10.0], radius: 1.0 };
        let circle_3 = Circle { centre: vector![10.0, 70.0], radius: 1.0 };
        let circle_4 = Circle { centre: vector![70.0, 70.0], radius: 1.0 };
        let circle_5 = Circle { centre: vector![50.0, 50.0], radius: 1.0 }; // spans all 4 quads
        let circle_6 = Circle { centre: vector![50.0, 25.0], radius: 1.0 }; // spans 0&1 quads
        let circle_7 = Circle { centre: vector![25.0, 50.0], radius: 1.0 }; // spans 0&2 quads
        let circle_8 = Circle { centre: vector![75.0, 50.0], radius: 1.0 }; // spans 1&3 quads
        let circle_9 = Circle { centre: vector![50.0, 75.0], radius: 1.0 }; // spans 2&3 quads

        let root_idx = 0;

        let br_idx_1 = qt_1.get_branch_idx(root_idx, circle_1).expect("no index returned for circle_1!");
        let br_idx_2 = qt_1.get_branch_idx(root_idx, circle_2).expect("no index returned for circle_2!");
        let br_idx_3 = qt_1.get_branch_idx(root_idx, circle_3).expect("no index returned for circle_3!");
        let br_idx_4 = qt_1.get_branch_idx(root_idx, circle_4).expect("no index returned for circle_4!");
        let br_idx_5 = qt_1.get_branch_idx(root_idx, circle_5);
        let br_idx_6 = qt_1.get_branch_idx(root_idx, circle_6);
        let br_idx_7 = qt_1.get_branch_idx(root_idx, circle_7);
        let br_idx_8 = qt_1.get_branch_idx(root_idx, circle_8);
        let br_idx_9 = qt_1.get_branch_idx(root_idx, circle_9);

        assert_eq!(0, br_idx_1);
        assert_eq!(1, br_idx_2);
        assert_eq!(2, br_idx_3);
        assert_eq!(3, br_idx_4);
        assert!(br_idx_5.is_none());
        assert!(br_idx_6.is_none());
        assert!(br_idx_7.is_none());
        assert!(br_idx_8.is_none());
        assert!(br_idx_9.is_none());

        // test box with min not at origin
        let bounding_box = AABB {
            min: vector![50.0, 50.0],
            max: vector![100.0, 100.0],
        };

        let qt_2 = QuadTree::new(bounding_box);

        let circle_10 = Circle { centre: vector![62.5, 68.75], radius: 4.0 };  // quad 0
        let circle_11 = Circle { centre: vector![75.0, 75.0], radius: 1.0 };   // spans all quads

        let br_idx_10 = qt_2.get_branch_idx(root_idx, circle_10).expect("no index returned for circle_10!");
        let br_idx_11 = qt_2.get_branch_idx(root_idx, circle_11);

        assert_eq!(0, br_idx_10);
        assert!(br_idx_11.is_none());
    }

    /// Returns the index of a node found by travelling depthwise into the tree, quadrant by quadrant,
    /// using the given list of quadrants.
    fn find_node_idx(qt: &QuadTree, quadrants: Vec<usize>) -> usize {
        let mut node_idx = qt.root;
        for (i, q) in quadrants.iter().enumerate() {
            node_idx = qt.get_node(node_idx).unwrap_or_else(|| panic!("no L{} node!", i))
                .children[*q].unwrap_or_else(|| panic!("no L{} Q{} index!", i, q));
        }

        node_idx
    }

    #[test]
    fn test_quadtree_construction() {
        let bounding_box = AABB {
            min: vector![0.0, 0.0],
            max: vector![100.0, 100.0],
        };

        let mut qt = QuadTree::new(bounding_box);

        let mut obj_list = Vec::new();
        for _ in 0..3 {
            let rnd_pt = vector![10.0, 10.0];

            let obj = (Circle { centre: rnd_pt, radius: 3.0 },
                       1);

            obj_list.push(obj);
        }

        for obj in obj_list {
            qt.insert(obj.0, obj.1);
        }

        // check placement
        let mut node_idx;

        // entity index 1 should be in the 5th level,
        // L0 Q0 -> L1 Q0 -> L2 Q0 -> L3 Q0 -> L4 Q0 -> L5 location
        qt.insert(Circle { centre: vector![1.0, 1.0], radius: 2.0 }, 1);

        node_idx = find_node_idx(&qt, vec![0, 0, 0, 0, 0]);

        assert!(qt.get_node(node_idx).expect("no L5 destination node!").entities.contains(&1));

        // entity index 2 should be in 0th level
        qt.insert(Circle { centre: vector![50.0, 50.0], radius: 0.01 }, 2);

        node_idx = find_node_idx(&qt, vec![]);

        assert!(qt.get_node(node_idx).expect("no root!") .entities.contains(&2));

        // entity index 3 should be in the 2nd level, L0 Q3 -> L1 Q0 -> L2 location
        qt.insert(Circle { centre: vector![68.75, 62.5], radius: 4.00 }, 3);

        node_idx = find_node_idx(&qt, vec![3, 0]);

        assert!(qt.get_node(node_idx).expect("no L2 destination node!").entities.contains(&3));
    }

    /// Fills every node in the tree with a single centred entity index down to the given depth.
    /// The span is the width of the root bounding box.  The bounding box is assumed start have a
    /// min at [0, 0] and be square.
    fn fill_tree(qt: &mut QuadTree, span: f64, depth: i32) {
        let mut step = span;
        let mut x_min;
        let mut x_max;
        let mut y_min;
        let mut y_max;
        let mut ent_idx = 0;
        let min_quadrant_span = span / 2.0f64.powi(depth);

        while step >= min_quadrant_span {
            y_min = 0.0;
            y_max = step;

            while y_max <= span {
                x_min = 0.0;
                x_max = step;

                while x_max <= span {
                    qt.insert(Circle {
                                  centre: vector![(x_min + x_max) / 2.0, (y_min + y_max) / 2.0],
                                  radius: 0.01,
                              },
                              ent_idx);

                    println!("min: [{}, {}], max: [{}, {}]", x_min, y_min, x_max, y_max);
                    ent_idx += 1;
                    x_min += step;
                    x_max += step;
                }

                y_min += step;
                y_max += step;
            }

            step /= 2.0;
        }
    }

    #[test]
    fn test_traversal() {
        let bounding_box = AABB {
            min: vector![0.0, 0.0],
            max: vector![100.0, 100.0],
        };

        let mut qt = QuadTree::new(bounding_box);

        fill_tree(&mut qt, 100.0, 5);  // complete tree down to max depth.

        // do a preorder traversal of the tree checking that there is exactly one entity id
        // contained in every node.
        let mut iter = qt.preorder_iter();

        let mut node_count = 0;
        while let Some(idx) = iter.next(&qt) {
            let node = qt.get_node(idx).unwrap();
            node_count += 1;

            println!("Node idx: {}", idx);
            println!("Node boundary: {:?}", node.boundary);
            println!("Node entities: {:?}", node.entities);

            assert_eq!(1, node.entities.len());
        }

        // ensure the correct number of nodes were found for a complete depth 5 tree.
        assert_eq!(1365, node_count);  // g.p. sum[0;n-1]{4^k} = (1 - 4^n) / (1 - 4)

        // check that nodes are retrieved in the correct order.

        // smaller depth 2 complete tree
        qt = QuadTree::new(bounding_box);

        fill_tree(&mut qt, 100.0, 2);
        
        iter = qt.preorder_iter();

        let indices = vec![0, 1, 5, 6, 9, 10, 2, 7, 8, 11, 12, 3, 13, 14, 17, 18, 4, 15, 16, 19, 20];

        node_count = 0;
        while let Some(idx) = iter.next(&qt) {
            let node = qt.get_node(idx).unwrap();

            println!("Node idx: {}", idx);
            println!("Node boundary: {:?}", node.boundary);
            println!("Node entities: {:?}", node.entities);

            assert_eq!(1, node.entities.len());
            assert_eq!(indices[node_count], node.entities[0]);  // check correct entity index is found
            node_count += 1;
        }

        assert_eq!(21, node_count);  // g.p. sum[0;n-1]{4^k} = (1 - 4^n) / (1 - 4), n=depth-1
    }
}
