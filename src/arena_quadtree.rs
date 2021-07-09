use nalgebra::{
    vector,
    Vector2,
};

use crate::{
    shapes::{self, Aabb, Circle, Plane},
};

/// The unique positional index of a node in the QuadTree's arena. Used to retrieve nodes.
pub type QTIndex = usize;

/// Each node has four child nodes that correspond to subdivision of the node's boundary into equal
/// quadrants. This enum is used to lookup a child node based on a specific quadrant.
#[derive(Clone, Copy, Debug)]
pub enum ChildQuadrant {
    TopLeft = 0,
    TopRight = 1,
    BottomLeft = 2,
    BottomRight = 3,
}

impl ChildQuadrant {
    // All possible values of the enum.
    const VALUES: [Self; 4] = [Self::TopLeft, Self::TopRight, Self::BottomLeft, Self::BottomRight];

    /// Returns a unit vector describing the offset direction of a child node's centre from its
    /// parent's centre.
    pub fn centre_offset(&self) -> Vector2<f64> {
        match self {
            Self::TopLeft => vector![-1.0, -1.0],
            Self::TopRight => vector![1.0, -1.0],
            Self::BottomLeft => vector![-1.0, 1.0],
            Self::BottomRight => vector![1.0, 1.0],
        }
    }
}

/// Each node has a unique index that locates it within the quadtree arena, an outer boundary as a
/// centre position and axis-aligned bounding box, a vector list of stored data and an array of 4
/// optional child node indices.
#[derive(Debug)]
pub struct Node<T: Copy> {
    idx: usize,
    centre: Vector2<f64>,
    boundary: Aabb,
    data: Vec<T>,
    children: [Option<QTIndex>; 4],
}

impl<T: Copy> Node<T> {
    /// Creates an empty node with the given index, center position and outer boundary.
    fn new(idx: usize, centre: Vector2<f64>, boundary: Aabb) -> Self {
        Self {
            idx,
            centre,
            boundary,
            data: vec![],
            children: Default::default(),
        }
    }

    /// Returns the index of this node.
    pub fn get_idx(&self) -> QTIndex {
        self.idx
    }

    /// Returns a vector list of data stored in this node.
    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    /// Returns an array of options containing indices of child nodes connected to this node.
    pub fn get_child_indices(&self) -> [Option<QTIndex>; 4] {
        self.children
    }

    /// Returns an option containing the index of the child node requested by its quadrant
    /// position, or None if there is no connection.
    pub fn get_child_idx(&self, c: ChildQuadrant) -> Option<QTIndex> {
        self.children[c as usize]
    }

    /// Creates a connection from the node to a child node via the given quadrant.
    pub fn set_child_idx(&mut self, quadrant: ChildQuadrant, child_node_idx: QTIndex) {
        self.children[quadrant as usize] = Some(child_node_idx);
    }
}

/// An arena based, statically sized QuadTree implementation that stores nodes in a vector. It is
/// used to house data that implements the Copy trait.
#[derive(Debug, Default)]
pub struct QuadTree<T: Copy> {
    arena: Vec<Node<T>>,
    root: QTIndex,
}

impl<T: Copy> QuadTree<T> {
    const MAX_DEPTH: i32 = 5;

    /// Creates a new empty QuadTree.
    pub fn new() -> Self {
        Self {
            arena: vec![],
            root: 0,
        }
    }

    /// Preallocates a quadtree down to the maximum depth, within the given boundary and centred at
    /// the given position.
    pub fn initialize(&mut self, centre: Vector2<f64>, boundary: Aabb) {
        // Recursive helper function that builds out the tree down to the given depth.
        fn helper<T: Copy>(qt: &mut QuadTree<T>, depth: i32, centre: Vector2<f64>, boundary: Aabb) -> Option<QTIndex> {
            // Depth limit has been reached.
            if depth < 0 {
                return None;
            }

            // Add new node to arena.
            let idx = qt.arena.len();
            qt.arena.push(Node::new(idx, centre, boundary));

            // Populate node's children by subdividing current node into equal quarters.
            for c in ChildQuadrant::VALUES.iter() {
                let offset = vector![
                    c.centre_offset().x * 0.5 * boundary.extents().x,
                    c.centre_offset().y * 0.5 * boundary.extents().y
                ];

                let child_centre = centre + offset;
                let child_boundary = Aabb::new(boundary.extents().x * 0.5,
                                               boundary.extents().y * 0.5);

                qt.arena[idx].children[*c as usize] = helper(qt,
                                                             depth - 1,
                                                             child_centre,
                                                             child_boundary);
            }

            Some(idx)
        }

        if let Some(root_idx) = helper(self, Self::MAX_DEPTH, centre, boundary) {
            self.root = root_idx;
        }
    }

    /// Returns the index of the root node.
    pub fn get_root_idx(&self) -> QTIndex {
        self.root
    }

    /// Returns an option; either Some() containing a reference to the root node, or None if there
    /// is no root node (i.e. the tree has not been initialised).
    pub fn get_root_node(&self) -> Option<&Node<T>> {
        self.get_node(self.root)
    }

    /// Returns a reference to the node located at the given arena index contained within an
    /// Option. Returns None if the node is not found.
    pub fn get_node(&self, idx: QTIndex) -> Option<&Node<T>> {
        self.arena.get(idx)
    }

    /// Calculates the child branch of a node that a given shape fits in.
    ///
    /// The given node's outer bounding box is split into 4 equal quadrants and the shape's
    /// position and dimensions are used to determine which of these (if any) it wholly fits
    /// within, whilst still allowing overlap of the perimeter.
    ///
    /// The quadrants are indexed as follows, assuming an x-y coord system with an origin at the
    /// top left, x-axis that increases positively left->right and y-axis up->down:
    ///
    /// Top left: 0           (0) | (1)
    /// Top right: 1         -----------
    /// Bottom left: 2        (2) | (3)
    /// Bottom right 3
    ///
    /// If the shape does not fit wholly within any quadrant (allowing for outer perimeter
    /// overlap), None is returned.
    fn get_child_quadrant(&self, node_idx: QTIndex, shape: Circle, shape_centre: Vector2<f64>) -> Option<usize> {
        let mut idx = 0;

        let node = &self.arena[node_idx];

        let delta = shape_centre - node.centre;

        if (delta.x.abs() <= shape.radius()) || (delta.y.abs() <= shape.radius()) {
            return None;  // straddles multiple child nodes, or max depth reached.
        }

        for (i, d) in delta.iter().enumerate() {
            if *d > 0.0 { idx |= 1 << i }
        }
        
        Some(idx)
    }

    // TODO return error if shape outside the bounds of the tree?
    /// Inserts the given data into the tree according to its associated shape and position.
    pub fn insert(&mut self, shape: Circle, shape_pos: Vector2<f64>, data: T) {
        let mut node_idx = self.root;  // start at root

        // Traverse down the branch, stopping if the shape will not fit within a child node, or a
        // leaf has been reached.
        while let Some(child_quadrant) = self.get_child_quadrant(node_idx, shape, shape_pos) {
            let node = &self.arena[node_idx];

            if let Some(child_idx) = node.children[child_quadrant] {
                // child exists so update node_idx and continue down branch.
                node_idx = child_idx;
            } else {
                // Reached leaf, stop.
                break;
            }
        }

        // Finally insert the data at the reached tree node.
        self.arena[node_idx].data.push(data);
    }

    /// Returns all data entries in the quad tree that reside in nodes intersected by the given
    /// plane.
    pub fn query_by_plane(&self, plane: &Plane, plane_pos: &Vector2<f64>) -> Vec<T> {
        // Determines whether the plane intersects the current node and then recursively checks any
        // connected child nodes, collecting any data entries found in intersected nodes along
        // the way.
        fn helper<T: Copy>(qt: &QuadTree<T>, node_idx: QTIndex, plane: &Plane, plane_pos: &Vector2<f64>, result: &mut Vec<T>) {
            let node = qt.get_node(node_idx).unwrap();
            let aabb = node.boundary;
            let aabb_pos = node.centre;

            if shapes::aabb_plane_are_intersecting(&aabb, &aabb_pos, plane, &plane_pos) {
                // collect data entries.
                for d in node.data.iter() {
                    result.push(*d);
                }
                // recurse to child nodes.
                for child_idx in node.children.iter().flatten() {
                    helper(qt, *child_idx, plane, plane_pos, result);
                }
            }
        }

        let mut result = vec![];

        helper(&self, self.root, plane, plane_pos, &mut result);

        result
    }

    /// Returns a custom iterator that performs a preorder traversal of the tree. The 'next'
    /// function of the iterator must be manually called and passed a reference to the tree, thus
    /// allowing tree mutation between calls to next.
    pub fn preorder_iter(&self) -> QTPreorderIter {
        QTPreorderIter::new(self.root)
    }
}

/// A custom iterator that performs a preorder traversal of the tree.
pub struct QTPreorderIter {
    stack: Vec<QTIndex>,
}

impl QTPreorderIter {
    /// Returns a new iterator. Requires the root index.
    pub fn new(root: QTIndex) -> Self {
        Self {
            stack: vec![root],
        }
    }

    /// Returns the next node index in the preorder traversal of the tree. Takes a reference to the
    /// tree each time it is called to allow tree mutations between calls.
    pub fn next<T: Copy>(&mut self, quad_tree: &QuadTree<T>) -> Option<QTIndex> {
        if let Some(node_idx) = self.stack.pop() {
            if let Some(node) = quad_tree.get_node(node_idx) {
                // push the child node idx on to the stack in reverse order so that they are
                // served back in ascending quadrant order.
                for child_idx in node.children.iter().rev().flatten() {
                    self.stack.push(*child_idx);
                }
            }
            return Some(node_idx);
        }
        None
    }
}

#[cfg(test)]
mod test {
    use rand::prelude::*;
    use specs::world::Index;
    use super::*;

    /// Fills every node in the tree, down to the given depth, with a single u32 data entry
    /// associated with a centred shape (relative to each node).
    /// The span is the width (or height) of the root bounding box. The bounding box is assumed
    /// to have a min at [0, 0] and be square.
    fn fill_tree(qt: &mut QuadTree<u32>, span: f64, depth: i32) {
        let mut step = span;
        let mut x_min;
        let mut x_max;
        let mut y_min;
        let mut y_max;
        let mut data = 0;
        let min_quadrant_span = span / 2.0f64.powi(depth);

        while step >= min_quadrant_span {
            y_min = 0.0;
            y_max = step;

            while y_max <= span {
                x_min = 0.0;
                x_max = step;

                while x_max <= span {
                    qt.insert(Circle::new(0.01),
                              vector![(x_min + x_max) / 2.0, (y_min + y_max) / 2.0],
                              data);

                    data += 1;
                    x_min += step;
                    x_max += step;
                }

                y_min += step;
                y_max += step;
            }

            step /= 2.0;
        }
    }

    /// Returns the index of a node found by travelling depthwise into the tree, quadrant by quadrant,
    /// using the given list of quadrants.
    fn find_node_idx_by_quadrants(qt: &QuadTree<Index>, quadrants: Vec<usize>) -> QTIndex {
        let mut node_idx = qt.root;
        for (i, q) in quadrants.iter().enumerate() {
            node_idx = qt.get_node(node_idx).unwrap_or_else(|| panic!("no L{} node!", i))
                .children[*q].unwrap_or_else(|| panic!("no L{} Q{} index!", i, q));
        }

        node_idx
    }

    #[test]
    fn test_query_by_plane_precision() {
        // 100.0 x 100.0 bounding box.
        let width = 100;
        let height = 100;
        let min_x = width as f64 * -0.5;
        let min_y = height as f64 * -0.5;
        let bounding_box = Aabb::new(width as f64 * 0.5, height as f64 * 0.5);
        let centre = vector![0.0, 0.0];

        let mut qt = QuadTree::new();
        qt.initialize(centre, bounding_box);

        // Insert a single entry at the very edge of the tree.
        let c = Circle::new(10.0);
        let c_pos = vector![-40.0, -40.0];
        let data = 9;

        qt.insert(c, c_pos, data);

        // Plane coincident with the top boundary of the quadtree.
        let plane = Plane::new(vector![0.0, 1.0]);
        let plane_pos = vector![min_x, min_y];

        let mut result_list = qt.query_by_plane(&plane, &plane_pos);

        assert_eq!(1, result_list.len());

        assert_eq!(data, result_list.pop().unwrap());
    }

    #[test]
    fn test_query_by_plane() {
        // 100.0 x 100.0 bounding box.
        let bounding_box = Aabb::new(50.0, 50.0);
        let centre = vector![50.0, 50.0];

        let mut qt = QuadTree::new();
        qt.initialize(centre, bounding_box);

        let depth = 5;
        fill_tree(&mut qt, 100.0, depth);  // complete tree filled down to max depth.

        // Plane coincident with the top boundary of the quadtree.
        let plane = Plane::new(vector![0.0, 1.0]);
        let plane_pos = vector![0.0, 0.0];

        let mut result_list = qt.query_by_plane(&plane, &plane_pos);

        // There should be 2^(max_depth + 1) - 1 data entries in the result list.
        assert_eq!(63, result_list.len());

        // Generate expected indices for top quadrants at each depth level.
        // 0 ... 1,2 ... 5,6,7,8 ... 21,22,23,24,25,26,27,28 ... etc.
        let mut expected = vec![];
        let mut start = 0;
        for d in 0..depth {
            for i in start..start + 2u32.pow(d as u32) {
                expected.push(i);
            }
            start += 4u32.pow(d as u32);
        }

        // Sort so we can compare with expected list which is in numerical order.
        result_list.sort();

        for (exp, actual) in expected.iter().zip(result_list.iter()) {
            assert_eq!(exp, actual);
        }
    }

    #[test]
    fn test_branch_indexing() {
        // 100.0 x 100.0 bounding box.
        let bounding_box = Aabb::new(50.0, 50.0);
        let centre = vector![50.0, 50.0];

        let mut qt_1: QuadTree<Index> = QuadTree::new();
        qt_1.initialize(centre, bounding_box);

        let radius = 1.0; 

        // circles wholly within a single quadrant.
        let enclosed_circle_pos_list = vec![
            vector![10.0, 10.0], // within quad 0
            vector![70.0, 10.0], // within quad 1
            vector![10.0, 70.0], // within quad 2
            vector![70.0, 70.0], // within quad 3
        ];

        // circles that will span 2 or more quadrants.
        let spanning_circle_pos_list = vec![
            vector![50.0, 50.0], // spans all 4 quads
            vector![50.0, 25.0], // spans 0&1 quads
            vector![25.0, 50.0], // spans 0&2 quads
            vector![75.0, 50.0], // spans 1&3 quads
            vector![50.0, 75.0]  // spans 2&3 quads
        ];

        // starting at the root, get the branch index for each circle.
        let root_idx = qt_1.get_root_idx();

        let mut indices_enclosed = vec![];
        for pos in enclosed_circle_pos_list {
            indices_enclosed.push(qt_1.get_child_quadrant(root_idx, Circle::new(radius), pos)
                                 .expect("no index returned for circle!"));
        }

        let mut indices_spanning = vec![];
        for pos in spanning_circle_pos_list {
            indices_spanning.push(qt_1.get_child_quadrant(root_idx, Circle::new(radius), pos));
        }

        let indices_enclosed_expected = vec![0, 1, 2, 3];

        for (expected, actual) in indices_enclosed_expected.iter().zip(indices_enclosed.iter()) {
            assert_eq!(expected, actual);
        }

        for actual_option in indices_spanning.iter() {
            assert!(actual_option.is_none());
        }

        // test box with min not at origin: 50 x 50 box
        let bounding_box = Aabb::new(25.0, 25.0);
        let centre = vector![75.0, 75.0];

        let mut qt_2: QuadTree<Index> = QuadTree::new();
        qt_2.initialize(centre, bounding_box);

        let root_idx = qt_2.get_root_idx();

        // should be wholly contained in quadrant 0 (TopLeft).
        let ch_quad_enc = qt_2.get_child_quadrant(root_idx, Circle::new(4.0), vector![62.5, 68.75])
                              .expect("no index returned for circle!");
        assert_eq!(0, ch_quad_enc);

        // should span all quadrants and therefore return None.
        let ch_quad_span = qt_2.get_child_quadrant(root_idx, Circle::new(1.0), vector![75.0, 75.0]);

        assert!(ch_quad_span.is_none());
    }

    #[test]
    fn test_quadtree_construction() {
        // 100.0 x 100.0 bounding box.
        let bounding_box = Aabb::new(50.0, 50.0);
        let centre = vector![50.0, 50.0];

        let mut qt = QuadTree::new();
        qt.initialize(centre, bounding_box);

        // fill with random data
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let c = Circle::new(rng.gen_range(1.0..20.0));
            let c_pos = vector![rng.gen_range(0.0..100.0), rng.gen_range(0.0..100.0)];
            let data = rng.gen_range(4..10_000);
            qt.insert(c, c_pos, data);
        }

        // check placement
        let mut node_idx;
        let mut node;

        // entity index 1 should be in the 5th level,
        // L0 Q0 -> L1 Q0 -> L2 Q0 -> L3 Q0 -> L4 Q0 -> L5 location
        qt.insert(Circle::new(2.0), vector![1.0, 1.0], 1);

        node_idx = find_node_idx_by_quadrants(&qt, vec![0, 0, 0, 0, 0]);
        node = qt.get_node(node_idx).expect("no L5 destination node!");

        assert!(node.data.contains(&1));

        // entity index 2 should be in 0th level
        qt.insert(Circle::new(0.01), vector![50.0, 50.0], 2);

        node_idx = find_node_idx_by_quadrants(&qt, vec![]);
        node = qt.get_node(node_idx).expect("no root!");

        assert!(node.data.contains(&2));

        // entity index 3 should be in the 2nd level, L0 Q3 -> L1 Q0 -> L2 location
        qt.insert(Circle::new(4.0), vector![68.75, 62.5], 3);

        node_idx = find_node_idx_by_quadrants(&qt, vec![3, 0]);
        node = qt.get_node(node_idx).expect("no L2 destination node!");

        assert!(node.data.contains(&3));
    }

    #[test]
    fn test_traversal() {
        // 100.0 x 100.0 bounding box.
        let bounding_box = Aabb::new(50.0, 50.0);
        let centre = vector![50.0, 50.0];

        let mut qt = QuadTree::new();
        qt.initialize(centre, bounding_box);

        fill_tree(&mut qt, 100.0, 5);  // complete tree filled down to max depth.

        // do a preorder traversal of the tree checking that there is exactly one data entry
        // contained in every node.
        let mut iter = qt.preorder_iter();

        let mut node_count = 0;
        while let Some(idx) = iter.next(&qt) {
            let node = qt.get_node(idx).unwrap();
            node_count += 1;

            println!("Node idx: {}", idx);
            println!("Node boundary: {:?}", node.boundary);
            println!("Node centre: {:?}", node.centre);
            println!("Node data: {:?}", node.data);

            assert_eq!(1, node.data.len());
        }

        // ensure the correct number of nodes were found for a complete depth 5 tree.
        assert_eq!(1365, node_count);  // g.p. sum[0..n-1]{4^k} = (1 - 4^n) / (1 - 4)

        // check that nodes are retrieved in the correct order.

        // tree filled down to depth 2 only.
        qt = QuadTree::new();
        qt.initialize(centre, bounding_box);

        fill_tree(&mut qt, 100.0, 2);
        
        iter = qt.preorder_iter();

        let expected = vec![
            0, 1, 5, 6, 9, 10, 2, 7, 8, 11, 12,
            3, 13, 14, 17, 18, 4, 15, 16, 19, 20
        ];

        node_count = 0;
        while let Some(idx) = iter.next(&qt) {
            let node = qt.get_node(idx).unwrap();

            // Check nodes down to depth 2 only.
            if node.boundary.extents().x >= 12.5 {
                println!("Node idx: {}", idx);
                println!("Node boundary: {:?}", node.boundary);
                println!("Node centre: {:?}", node.centre);
                println!("Node data: {:?}", node.data);

                assert_eq!(1, node.data.len());
                assert_eq!(expected[node_count], node.data[0]);  // check correct entity index is found

                node_count += 1;
            }
        }

        assert_eq!(21, node_count);  // g.p. sum[0;n-1]{4^k} = (1 - 4^n) / (1 - 4), n=depth-1
    }
}
