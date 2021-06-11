use nalgebra::{
    base::Vector2,
    vector,
};
use specs::{
    world::Index,
};

//const DEFAULT_BUCKET_SIZE: usize = 10; TODO possible extension

/// A tree structure used for spatial partitioning and tracking of entities.
pub struct QuadTree {
    root: Node,
}

impl QuadTree {
    /// Creates an empty QuadTree with the given boundary.
    pub fn new(bounding_box: AABB) -> Self {
        Self { root: Node::new(bounding_box, 0) }
    }

    /// Inserts a Circle object with the given index into the QuadTree.
    /// The circle's parameters determine its location in the tree and the index is a unique
    /// identifier for a tracked entity.
    pub fn insert(&mut self, object: Circle, obj_index: Index) {
        self.root.insert(object, obj_index);
    }
}

//
// Child nodes arranged as follows, with coords origin at top left corner:
//
//     ---------
//     : 0 : 1 :
//     ---------
//     : 2 : 3 :
//     ---------
//
#[derive(Debug)]
struct Node {
    bounding_box: AABB,
    children: [Option<Box<Node>>; 4],
    objects: Option<Vec<(Circle, Index)>>,
    depth: usize,
}

impl Node {
    const MAX_DEPTH: usize = 5;

    /// Creates a new Quadtree node with the given boundary and marks its depth.
    fn new(bounding_box: AABB, depth: usize) -> Self {
        Self {
            bounding_box,
            children: [None, None, None, None],
            objects: None,
            depth,
        }
    }

    /// Inserts an item into the tree based on its bounding box dimensions.
    fn insert(&mut self, object: Circle, obj_index: Index) {
        // Determine appropriate child node to place object in.
        if let Some(index) = self.get_child_index(object) {
            match self.children[index].as_mut() {
                Some(child_node) => {
                    child_node.insert(object, obj_index);
                    return;
                }

                // If child node doesn't exist yet, create it.
                None => {
                    // Half-diagonal of bounding box.
                    let span = (self.bounding_box.max - self.bounding_box.min) * 0.5;

                    let child_min = match index {
                        0 => self.bounding_box.min,
                        1 => self.bounding_box.min + vector![span.x, 0.0],
                        2 => self.bounding_box.min + vector![0.0, span.y],
                        3 => self.bounding_box.min + span,
                        _ => panic!("index out of range"),
                    };

                    let child_max = child_min + span;

                    let mut new_node = Node::new(AABB { min: child_min, max: child_max },
                                             self.depth + 1);

                    new_node.insert(object, obj_index);

                    self.children[index] = Some(Box::new(new_node));
                }
            }
        } else {  // cannot be placed into a child node, so place in current node.
            if let Some(obj_list) = self.objects.as_mut() {
                obj_list.push((object, obj_index));
            } else {
                let new_list = vec![(object, obj_index)];
                
                self.objects = Some(new_list);
            }
        }
    }

    /// Returns the index of the child node corresponding to the given Circle object's position
    /// and dimensions.
    fn get_child_index(&self, object: Circle) -> Option<usize> {
        let mut index = 0;

        // delta vector from object centre to bounding box centre.
        let bounding_box_centre = self.bounding_box.min + 0.5 * (self.bounding_box.max
                                                                 - self.bounding_box.min);
        let delta = object.centre - bounding_box_centre;

        if (self.depth >= Self::MAX_DEPTH) || (delta.x.abs() <= object.radius) || (delta.y.abs() <= object.radius) {
            return None;  // straddles multiple child nodes, or max depth reached.
        }

        for (i, d) in delta.iter().enumerate() {
            if *d > 0.0 { index |= 1 << i }
        }

        Some(index)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Circle {
    centre: Vector2<f64>,
    radius: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct AABB {
    min: Vector2<f64>,
    max: Vector2<f64>,
}

pub enum ChildIndex {
    NW = 0,
    NE = 1,
    SW = 2,
    SE = 3,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_indexing() {
        let bounding_box = AABB {
            min: vector![0.0, 0.0],
            max: vector![100.0, 100.0],
        };

        let qt_1 = Node::new(bounding_box, 0);

        let circle_1 = Circle { centre: vector![10.0, 10.0], radius: 1.0 };
        let circle_2 = Circle { centre: vector![70.0, 10.0], radius: 1.0 };
        let circle_3 = Circle { centre: vector![10.0, 70.0], radius: 1.0 };
        let circle_4 = Circle { centre: vector![70.0, 70.0], radius: 1.0 };
        let circle_5 = Circle { centre: vector![50.0, 50.0], radius: 1.0 }; // spans all 4 quads
        let circle_6 = Circle { centre: vector![50.0, 25.0], radius: 1.0 }; // spans 0&1 quads
        let circle_7 = Circle { centre: vector![25.0, 50.0], radius: 1.0 }; // spans 0&2 quads
        let circle_8 = Circle { centre: vector![75.0, 50.0], radius: 1.0 }; // spans 1&3 quads
        let circle_9 = Circle { centre: vector![50.0, 75.0], radius: 1.0 }; // spans 2&3 quads

        let idx_1 = qt_1.get_child_index(circle_1).expect("no index returned for circle_1!");
        let idx_2 = qt_1.get_child_index(circle_2).expect("no index returned for circle_2!");
        let idx_3 = qt_1.get_child_index(circle_3).expect("no index returned for circle_3!");
        let idx_4 = qt_1.get_child_index(circle_4).expect("no index returned for circle_4!");
        let idx_5 = qt_1.get_child_index(circle_5);
        let idx_6 = qt_1.get_child_index(circle_6);
        let idx_7 = qt_1.get_child_index(circle_7);
        let idx_8 = qt_1.get_child_index(circle_8);
        let idx_9 = qt_1.get_child_index(circle_9);

        assert_eq!(0, idx_1);
        assert_eq!(1, idx_2);
        assert_eq!(2, idx_3);
        assert_eq!(3, idx_4);
        assert!(idx_5.is_none());
        assert!(idx_6.is_none());
        assert!(idx_7.is_none());
        assert!(idx_8.is_none());
        assert!(idx_9.is_none());

        // test box with min not at origin
        let bounding_box = AABB {
            min: vector![50.0, 50.0],
            max: vector![100.0, 100.0],
        };

        let qt_2 = Node::new(bounding_box, 0);

        let circle_10 = Circle { centre: vector![62.5, 68.75], radius: 4.0 };  // quad 0
        let circle_11 = Circle { centre: vector![75.0, 75.0], radius: 1.0 };   // spans all quads

        let idx_10 = qt_2.get_child_index(circle_10).expect("no index returned for circle_10!");
        let idx_11 = qt_2.get_child_index(circle_11);

        assert_eq!(0, idx_10);
        assert!(idx_11.is_none());
    }

    #[test]
    fn test_quadtree_construction() {
        let bounding_box = AABB {
            min: vector![0.0, 0.0],
            max: vector![100.0, 100.0],
        };

        let mut qt = Node::new(bounding_box, 0);

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

        // should be in 5th level 0th index box
        let obj_1 = (Circle { centre: vector![1.0, 1.0], radius: 2.0 }, 1);
        qt.insert(obj_1.0, obj_1.1);

        assert!(qt.children[0].as_ref().expect("no L1 child!")  // level 1 quadrant 0
                .children[0].as_ref().expect("no L2 child!")    // level 2 quadrant 0
                .children[0].as_ref().expect("no L3 child!")    // level 3 quadrant 0
                .children[0].as_ref().expect("no L4 child!")    // level 4 quadrant 0
                .children[0].as_ref().expect("no L5 child!")    // level 5 quadrant 0
                .objects.as_ref().expect("no objects!")            // objects list
                .contains(&obj_1));

        // should be in 0th level
        let obj_2 = (Circle { centre: vector![50.0, 50.0], radius: 0.01 }, 2);
        qt.insert(obj_2.0, obj_2.1);

        assert!(qt.objects.as_ref().expect("no objects!")      // objects list at 0th level
                .contains(&obj_2));

        // should be in 0th level
        let obj_3 = (Circle { centre: vector![68.75, 62.5], radius: 4.00 }, 3);
        qt.insert(obj_3.0, obj_3.1);

        //println!("{:#?}", qt);
        assert!(qt.children[3].as_ref().expect("no L1 child!")  // level 1 quadrant 3
                .children[0].as_ref().expect("no L2 child!")    // level 2 quadrant 0
                .objects.as_ref().expect("no objects!")            // objects list
                .contains(&obj_3));

    }
}
