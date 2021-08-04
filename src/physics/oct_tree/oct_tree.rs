use bevy::math::DVec3;

use std::{
    collections::HashMap,
    hash::Hash,
};

use crate::{
    physics::components::PhysTransform,
    physics::oct_tree::{ChildOctant, OctIndex, node::OctTreeNode},
    physics::shapes::{self, *},
};

/// An arena based, statically sized oct-tree implementation that stores nodes in a vector. It is
/// used to house data that implements the Copy trait.
#[derive(Debug, Default)]
pub struct OctTree<T: Copy + Hash + Eq> {
    arena: Vec<OctTreeNode<T>>,
    data_node_map: HashMap<T, OctIndex>,
    root: OctIndex,
}

impl<T: Copy + Hash + Eq> OctTree<T> {
    const MAX_DEPTH: i32 = 5;

    /// Creates a new empty OctTree.
    pub fn new() -> Self {
        Self {
            arena: vec![],
            data_node_map: HashMap::new(),
            root: 0,
        }
    }

    /// Preallocates an oct-tree down to the maximum depth, within the given boundary and centred at
    /// the given position.
    pub fn initialize(&mut self, centre: DVec3, boundary: Aabb3D) {
        // Recursive helper function that builds out the tree down to the given depth.
        fn helper<T: Copy + Hash + Eq>(
            qt: &mut OctTree<T>,
            depth: i32,
            centre: DVec3,
            boundary: Aabb3D,
        ) -> Option<OctIndex> {
            // Depth limit has been reached.
            if depth < 0 {
                return None;
            }

            // Add new node to arena.
            let idx = qt.arena.len();
            qt.arena.push(OctTreeNode::new(idx, centre, boundary));

            // Populate node's children by subdividing current node into equal quarters.
            for c in ChildOctant::VALUES.iter() {
                let offset = DVec3::new(
                    c.centre_offset().x * 0.5 * boundary.extents().x,
                    c.centre_offset().y * 0.5 * boundary.extents().y,
                    c.centre_offset().z * 0.5 * boundary.extents().z,
                );

                let child_centre = centre + offset;
                let child_boundary = Aabb3D::new(
                    boundary.extents().x * 0.5,
                    boundary.extents().y * 0.5,
                    boundary.extents().z * 0.5,
                );

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
    pub fn get_root_idx(&self) -> OctIndex {
        self.root
    }

    /// Returns an option; either Some() containing a reference to the root node, or None if there
    /// is no root node (i.e. the tree has not been initialised).
    pub fn get_root_node(&self) -> Option<&OctTreeNode<T>> {
        self.get_node(self.root)
    }

    /// Returns a reference to the node located at the given arena index contained within an
    /// Option. Returns None if the node is not found.
    pub fn get_node(&self, idx: OctIndex) -> Option<&OctTreeNode<T>> {
        self.arena.get(idx)
    }

    /// Calculates the index of a node's child octant capable of wholly containing the given sphere.
    /// If the sphere does not fit wholly within any octant (allowing for outer perimeter
    /// overlap), None is returned.
    ///
    /// The given node's outer bounding box is split into 8 equal octants and the shape's
    /// position and dimensions are used to determine which of these (if any) it wholly fits
    /// within, whilst still allowing overlap of the outermost perimeter.
    //
    // The octants are indexed as follows. The two level diagrams represent the lower and upper
    // levels in the y-axis. Each level is set in the x- and z-axes as shown.
    //
    //     lower  (0) | (1)      \-------> x
    //       |   -----------     |
    //      y|    (2) | (3)      |
    //       |                   V
    //       V    (4) | (5)      z
    //     upper -----------
    //            (6) | (7)
    //
    fn calc_child_octant_idx(
        &self,
        node_idx: OctIndex,
        shape: &Sphere,
        shape_centre: DVec3,
    ) -> Option<usize> {
        let mut idx = 0;

        let node = &self.arena[node_idx];

        let delta = shape_centre - node.centre;

        if (delta.x.abs() <= shape.radius()) || (delta.y.abs() <= shape.radius())
            || (delta.z.abs() <= shape.radius()) {
            return None;  // straddles multiple child nodes, or max depth reached.
        }

        // TODO can use delta.to_array() in later glam version.
        for (i, d) in [delta.x, delta.y, delta.z].iter().enumerate() {
            if *d > 0.0 { idx |= 1 << i }
        }

        Some(idx)
    }

    // TODO return error if shape outside the bounds of the tree?
    /// Inserts the given data into the tree according to its associated shape and position.
    pub fn insert(&mut self, primative: &CollisionPrimative, transform: &PhysTransform, data: T) {
        let is_sphere = primative.0.is::<Sphere>();

        if is_sphere {
            self.insert_sphere(
                primative.0.downcast_ref::<Sphere>().unwrap(),
                transform.translation(),
                data
            );
        }
    }

    /// Specific insert method for sphere primatives.
    fn insert_sphere(&mut self, shape: &Sphere, shape_pos: DVec3, data: T) {
        let mut node_idx = self.root;  // start at root

        // Traverse down the branch, stopping if the shape will not fit within a child node, or a
        // leaf has been reached.
        while let Some(child_octant) = self.calc_child_octant_idx(node_idx, shape, shape_pos) {
            let node = &self.arena[node_idx];

            if let Some(child_idx) = node.children[child_octant] {
                // child exists so update node_idx and continue down branch.
                node_idx = child_idx;
            } else {
                // Reached leaf, stop.
                break;
            }
        }

        // Finally insert the data at the reached tree node.
        self.arena[node_idx].data.insert(data);

        // Record the node where this data entry is stored so it can be updated easily.
        self.data_node_map.insert(data, node_idx);
    }

    fn remove(&mut self, data: T) {
        let node_idx = match self.data_node_map.get(&data) {
            Some(idx) => idx,
            None => return, // no data to remove.
        };

        self.arena[*node_idx].data.remove(&data);
    }

    // TODO could be optimised to make better use of the knowledge of where the data is stored in
    // the tree. Assuming objects aren't moving quickly, it should be moved to a nearby node.
    /// Updates the location of the data point in the tree based on its current associated
    /// geometric position and spherical shape.
    pub fn update(&mut self, primative: &CollisionPrimative, transform: &PhysTransform, data: T) {
        let is_sphere = primative.0.is::<Sphere>();

        self.remove(data);

        if is_sphere {
            self.insert_sphere(
                primative.0.downcast_ref::<Sphere>().unwrap(),
                transform.translation(),
                data
            );
        }
    }

    /// Returns all data entries in the quad tree that reside in nodes intersected by the given
    /// plane.
    pub fn query_by_plane(&self, plane: &Plane, plane_pos: DVec3) -> Vec<T> {
        // Determines whether the plane intersects the current node and then recursively checks any
        // connected child nodes, collecting any data entries found in intersected nodes along
        // the way.
        fn helper<T: Copy + Hash + Eq>(
            qt: &OctTree<T>,
            node_idx: OctIndex,
            plane: &Plane,
            plane_pos: DVec3,
            result: &mut Vec<T>
        ) {
            let node = qt.get_node(node_idx).unwrap();
            let aabb = node.boundary;
            let aabb_pos = node.centre;

            if shapes::aabb_plane_are_intersecting(&aabb, aabb_pos, plane, plane_pos) {
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
    pub fn preorder_iter(&self) -> OctTreePreorderIter {
        OctTreePreorderIter::new(self.root)
    }
}

/// A custom iterator that performs a preorder traversal of the tree.
pub struct OctTreePreorderIter {
    stack: Vec<OctIndex>,
}

impl OctTreePreorderIter {
    /// Returns a new iterator. Requires the root index.
    pub fn new(root: OctIndex) -> Self {
        Self {
            stack: vec![root],
        }
    }

    /// Returns the next node index in the preorder traversal of the tree. Takes a reference to the
    /// tree each time it is called to allow tree mutations between calls.
    pub fn next<T: Copy + Hash + Eq>(&mut self, oct_tree: &OctTree<T>) -> Option<OctIndex> {
        if let Some(node_idx) = self.stack.pop() {
            if let Some(node) = oct_tree.get_node(node_idx) {
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
//    use specs::world::Index;
    use super::*;

    /// Fills every node in the tree, down to the given depth, with a single u32 data entry
    /// associated with a centred shape (relative to each node).
    /// The span is the width (or height) of the root bounding box. The bounding box is assumed
    /// to have a min at [0, 0] and be square.
    fn fill_tree(qt: &mut OctTree<u32>, span: f64, depth: i32) {
        let mut step = span;
        let mut x_min;
        let mut x_max;
        let mut y_min;
        let mut y_max;
        let mut z_min;
        let mut z_max;
        let mut data = 0;
        let min_quadrant_span = span / 2.0f64.powi(depth);

        while step >= min_quadrant_span {
            z_min = 0.0;
            z_max = step;

            while z_max <= span {
                y_min = 0.0;
                y_max = step;

                while y_max <= span {
                    x_min = 0.0;
                    x_max = step;

                    while x_max <= span {
                        qt.insert_sphere(
                            &Sphere::new(0.01),
                            DVec3::new(
                                (x_min + x_max) / 2.0,
                                (y_min + y_max) / 2.0,
                                (z_min + z_max) / 2.0,
                            ),
                            data,
                        );
                        data += 1;
                        x_min += step;
                        x_max += step;
                    }

                    y_min += step;
                    y_max += step;
                }

                z_min += step;
                z_max += step;
            }

            step /= 2.0;
        }
    }

    /// Returns the index of a node found by travelling depthwise into the tree, octant by octant,
    /// using the given list of octants.
    fn find_node_idx_by_octants(qt: &OctTree<usize>, octants: Vec<usize>) -> OctIndex {
        let mut node_idx = qt.root;
        for (i, q) in octants.iter().enumerate() {
            node_idx = qt.get_node(node_idx).unwrap_or_else(|| panic!("no L{} node!", i))
                .children[*q].unwrap_or_else(|| panic!("no L{} Q{} index!", i, q));
        }

        node_idx
    }

//    #[test]
//    fn test_query_by_plane_precision() {
//        // 100.0 x 100.0 bounding box.
//        let width = 100;
//        let height = 100;
//        let min_x = width as f64 * -0.5;
//        let min_y = height as f64 * -0.5;
//        let bounding_box = Aabb::new(width as f64 * 0.5, height as f64 * 0.5);
//        let centre = DVec3::new(0.0, 0.0];
//
//        let mut qt = QuadTree::new();
//        qt.initialize(centre, bounding_box);
//
//        // Insert a single entry at the very edge of the tree.
//        let c = Sphere::new(10.0);
//        let c_pos = DVec3::new(-40.0, -40.0];
//        let data = 9;
//
//        qt.insert_sphere(c, c_pos, data);
//
//        // Plane coincident with the top boundary of the quadtree.
//        let plane = Plane::new(DVec3::new(0.0, 1.0]);
//        let plane_pos = DVec3::new(min_x, min_y];
//
//        let mut result_list = qt.query_by_plane(&plane, &plane_pos);
//
//        assert_eq!(1, result_list.len());
//
//        assert_eq!(data, result_list.pop().unwrap());
//    }
//
//    #[test]
//    fn test_query_by_plane() {
//        // 100.0 x 100.0 bounding box.
//        let bounding_box = Aabb::new(50.0, 50.0);
//        let centre = DVec3::new(50.0, 50.0];
//
//        let mut qt = QuadTree::new();
//        qt.initialize(centre, bounding_box);
//
//        let depth = 5;
//        fill_tree(&mut qt, 100.0, depth);  // complete tree filled down to max depth.
//
//        // Plane coincident with the top boundary of the quadtree.
//        let plane = Plane::new(DVec3::new(0.0, 1.0]);
//        let plane_pos = DVec3::new(0.0, 0.0];
//
//        let mut result_list = qt.query_by_plane(&plane, &plane_pos);
//
//        // There should be 2^(max_depth + 1) - 1 data entries in the result list.
//        assert_eq!(63, result_list.len());
//
//        // Generate expected indices for top quadrants at each depth level.
//        // 0 ... 1,2 ... 5,6,7,8 ... 21,22,23,24,25,26,27,28 ... etc.
//        let mut expected = vec![];
//        let mut start = 0;
//        for d in 0..depth {
//            for i in start..start + 2u32.pow(d as u32) {
//                expected.push(i);
//            }
//            start += 4u32.pow(d as u32);
//        }
//
//        // Sort so we can compare with expected list which is in numerical order.
//        result_list.sort();
//
//        for (exp, actual) in expected.iter().zip(result_list.iter()) {
//            assert_eq!(exp, actual);
//        }
//    }

    #[test]
    fn test_calc_child_octant_idx() {
        // 100.0 x 100.0 bounding box.
        let bounding_box = Aabb3D::new(50.0, 50.0, 50.0);
        let centre = DVec3::new(50.0, 50.0, 50.0);

        let mut qt_1: OctTree<usize> = OctTree::new();
        qt_1.initialize(centre, bounding_box);

        let radius = 1.0;

        // spheres wholly within a single child octant of the root node.
        let enclosed_sphere_pos_list = vec![
            DVec3::new(10.0, 10.0, 10.0),   // within octant 0
            DVec3::new(70.0, 10.0, 10.0),   // within octant 1
            DVec3::new(10.0, 70.0, 10.0),   // within octant 2
            DVec3::new(70.0, 70.0, 10.0),   // within octant 3
            DVec3::new(10.0, 10.0, 70.0),   // within octant 4
            DVec3::new(70.0, 10.0, 70.0),   // within octant 5
            DVec3::new(10.0, 70.0, 70.0),   // within octant 6
            DVec3::new(70.0, 70.0, 70.0),   // within octant 7
        ];

        // spheres that will span 2 or more child octant of the root node.
        let mut spanning_sphere_pos_list = vec![];
        for i in -1_i8..1_i8 {
            for j in -1_i8..1_i8 {
                for k in -1_i8..1_i8 {
                    if i.abs() == 1 && j.abs() == 1 && k.abs() == 1 {
                        break;
                    }
                    spanning_sphere_pos_list.push(
                        DVec3::new(
                            50.0 + i as f64 * 25.0,
                            50.0 + j as f64 * 25.0,
                            50.0 + k as f64 * 25.0,
                    ));
                }
            }
        }

        // starting at the root, get the child octant index for each sphere.
        let root_idx = qt_1.get_root_idx();

        // check spheres enclosed within an octant return the correct index.
        let mut indices_enclosed: Vec<usize> = vec![];
        for pos in enclosed_sphere_pos_list {
            println!("pos {}, idx {}", pos, qt_1.calc_child_octant_idx(root_idx, &Sphere::new(radius), pos)
                                 .expect("no index returned for sphere!"));
            indices_enclosed.push(qt_1.calc_child_octant_idx(root_idx, &Sphere::new(radius), pos)
                                 .expect("no index returned for sphere!"));
        }

        let mut indices_spanning = vec![];
        for pos in spanning_sphere_pos_list {
            indices_spanning.push(qt_1.calc_child_octant_idx(root_idx, &Sphere::new(radius), pos));
        }

        let indices_enclosed_expected = vec![0, 1, 2, 3, 4, 5, 6, 7];

        for (expected, actual) in indices_enclosed_expected.iter().zip(indices_enclosed.iter()) {
            assert_eq!(expected, actual);
        }

        for option in indices_spanning.iter() {
            println!("{:?}", option);
            assert!(option.is_none());
        }

        // test box with min not at origin: 50 x 50 box
        let bounding_box = Aabb3D::new(25.0, 25.0, 25.0);
        let centre = DVec3::new(75.0, 75.0, 75.0);

        let mut qt_2: OctTree<usize> = OctTree::new();
        qt_2.initialize(centre, bounding_box);

        let root_idx = qt_2.get_root_idx();

        // should be wholly contained in octant 2.
        let child_octant_enc = qt_2.calc_child_octant_idx(
            root_idx,
            &Sphere::new(4.0),
            DVec3::new(62.5, 87.5, 68.75),
        )
        .expect("no index returned for sphere!");
        assert_eq!(2, child_octant_enc);

        // should span all octants and therefore return None.
        let child_octant_span = qt_2.calc_child_octant_idx(
            root_idx,
            &Sphere::new(1.0),
            DVec3::new(75.0, 75.0, 75.0)
        );

        assert!(child_octant_span.is_none());
    }

    #[test]
    fn test_octtree_construction() {
        // 100.0 x 100.0 bounding box.
        let bounding_box = Aabb3D::new(50.0, 50.0, 50.0);
        let centre = DVec3::new(50.0, 50.0, 50.0);

        let mut qt = OctTree::new();
        qt.initialize(centre, bounding_box);

        // fill with random data
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let c = Sphere::new(rng.gen_range(1.0..20.0));
            let c_pos = DVec3::new(
                rng.gen_range(0.0..100.0),
                rng.gen_range(0.0..100.0),
                rng.gen_range(0.0..100.0),
            );
            let data = rng.gen_range(4..10_000);
            qt.insert_sphere(&c, c_pos, data);
        }

        // check placement
        let mut node_idx;
        let mut node;

        // entity index 1 should be in the 5th level,
        // L0 Octant2 -> L1 Octant2 -> L2 Octant2 -> L3 Octant2 -> L4 Octant2 -> L5 location
        qt.insert_sphere(&Sphere::new(2.0), DVec3::new(1.0, 1.0, 1.0), 1);

        node_idx = find_node_idx_by_octants(&qt, vec![0, 0, 0, 0, 0]);
        node = qt.get_node(node_idx).expect("no L5 destination node!");

        assert!(node.data.contains(&1));

        // entity index 2 should be in 0th level
        qt.insert_sphere(&Sphere::new(0.01), DVec3::new(50.0, 50.0, 50.0), 2);

        node_idx = find_node_idx_by_octants(&qt, vec![]);
        node = qt.get_node(node_idx).expect("no root!");

        assert!(node.data.contains(&2));

        // entity index 3 should be in the 2nd level, L0 Q3 -> L1 Q0 -> L2 location
        qt.insert_sphere(&Sphere::new(4.0), DVec3::new(68.75, 62.5, 68.75), 3);

        node_idx = find_node_idx_by_octants(&qt, vec![7, 0]);
        node = qt.get_node(node_idx).expect("no L2 destination node!");

        assert!(node.data.contains(&3));
    }

    /// Calculates the total number of nodes that would be present in a complete oct-tree of the
    /// given depth.
    ///
    /// The geometric progression formula used is:
    ///
    ///     Sum[k=0..depth]{ 8^k } = (1 - 8^(n+1)) / (1 - 8)
    fn calc_total_nodes(depth: u32) -> i32 {
        (8_i32.pow(depth + 1) - 1) / 7
    }

    #[test]
    fn test_traversal() {
        // 100.0 x 100.0 bounding box.
        let bounding_box = Aabb3D::new(50.0, 50.0, 50.0);
        let centre = DVec3::new(50.0, 50.0, 50.0);

        let mut qt = OctTree::new();
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
        assert_eq!(calc_total_nodes(5), node_count);

        // check that nodes are retrieved in the correct order.

        // tree filled down to depth 2 only.
        qt = OctTree::new();
        qt.initialize(centre, bounding_box);

        fill_tree(&mut qt, 100.0, 2);

        iter = qt.preorder_iter();

        let expected = vec![
            0,
            1, 9, 10, 13, 14, 25, 26, 29, 30,
            2, 11, 12, 15, 16, 27, 28, 31, 32,
            3, 17, 18, 21, 22, 33, 34, 37, 38,
            4, 19, 20, 23, 24, 35, 36, 39, 40,
            5, 41, 42, 45, 46, 57, 58, 61, 62,
            6, 43, 44, 47, 48, 59, 60, 63, 64,
            7, 49, 50, 53, 54, 65, 66, 69, 70,
            8, 51, 52, 55, 56, 67, 68, 71, 72
        ];

        node_count = 0;
        while let Some(idx) = iter.next(&qt) {
            let node = qt.get_node(idx).unwrap();

            // Check nodes down to depth 2 only.
            if node.boundary.extents().x >= 12.5 {
                println!("Node idx: {}", idx);
                println!("Node boundary: {:?}", node.boundary);
                println!("Node centre: {:?}", node.centre);
                println!("Node data: {:?}\n", node.data);

                // should only be one data entry at each node
                assert_eq!(1, node.data.len());

                // check correct data entry is found
                assert!(node.data.contains(&expected[node_count as usize]));
                node_count += 1;
            }
        }

        assert_eq!(calc_total_nodes(2), node_count);  // g.p. sum[0;n-1]{4^k} = (1 - 4^n) / (1 - 4), n=depth-1
    }
}
