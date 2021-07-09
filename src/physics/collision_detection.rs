use nalgebra::{
    vector,
    Vector2,
};
use specs::{
    prelude::*,
    world::{EntitiesRes, Index},
};

use crate::{
    shapes::{self, Aabb},
    components::*,
    constants,
    arena_quadtree::{Node, QTIndex, QuadTree},
};

#[derive(SystemData)]
pub struct CollisionDetectionSysData<'a> {
    circle_collider: ReadStorage<'a, CircleCollider>,
    collision: WriteStorage<'a, Collision>,
    mass: ReadStorage<'a, Mass>,
    boundary_collider: ReadStorage<'a, BoundaryCollider>,
    polygon_collider: ReadStorage<'a, PolygonCollider>,
    position: WriteStorage<'a, Position>,
    // Resources
    entities: ReadExpect<'a, EntitiesRes>,
}

/// System that detects collisions (intersections) between entities with collider components and
/// creates corresponding collision entities for later resolution by another system.
pub struct CollisionDetectionSys {
    ancestor_stack: Vec<QTIndex>,
}

impl CollisionDetectionSys {
    pub fn new() -> Self {
        Self { ancestor_stack: vec![] }
    }

    /// Broad phase detection of possible collisions between entities. References a quadtree that
    /// stores entity indices by their associated outer bounding shape. Entities that are located
    /// on the same branch of the tree are passed as collision candidates to the next stage of the
    /// process.
    pub fn detect_all_internal_collisions(&mut self, qt: &QuadTree<Index>,
                                          data: &mut CollisionDetectionSysData) {
        fn helper(col_sys: &mut CollisionDetectionSys, qt: &QuadTree<Index>, node: &Node<Index>,
                  data: &mut CollisionDetectionSysData) {
            // add current node to ancestor stack
            col_sys.ancestor_stack.push(node.get_idx());

            for ancestor_idx in &col_sys.ancestor_stack {
                let ancestor_node = qt.get_node(*ancestor_idx)
                    .expect("Node indexed in ancestor stack not found!");
                let ancestor_objects = ancestor_node.get_data();

                for idx_a in ancestor_objects {
                    let current_objects = node.get_data();

                    for idx_b in current_objects {
                        // ignore self on self collisions and duplicates (i.e. 1->2 and 2->1).
                        if idx_a == idx_b { break; }

                        CollisionDetectionSys::detect_collision(idx_a, idx_b, data);
                    }
                }
            }

            // preorder tree traversal.
            for c in node.get_child_indices().iter() {
                if let Some(child_idx) = c {
                    let child_node = qt.get_node(*child_idx).unwrap();
                    helper(col_sys, qt, child_node, data);
                }
            }

            // remove current node from ancestor stack
            col_sys.ancestor_stack.pop();
        }

        let root_node = qt.get_root_node().expect("The QuadTree has not been initialised!");

        helper(self, qt, root_node, data);
    }

    /// Narrow phase collision detection. Tests intersection between the two entities corresponding
    /// to the given indices.
    fn detect_collision(idx_a: &Index, idx_b: &Index, data: &mut CollisionDetectionSysData) {
        let ent_a = data.entities.entity(*idx_a); // TODO Check is_alive()
        let ent_b = data.entities.entity(*idx_b);

        let pos_a = data.position.get(ent_a).expect("Collision entity lacks a position component.")
            .vector;
        let pos_b = data.position.get(ent_b).expect("Collision entity lacks a position component.")
            .vector;

        // Determine types of colliders and apply appropriate intersection test.
        match (data.circle_collider.get(ent_a),
               data.boundary_collider.get(ent_a),
               data.polygon_collider.get(ent_a),
               data.circle_collider.get(ent_b),
               data.boundary_collider.get(ent_b),
               data.polygon_collider.get(ent_b)) {
            (Some(ca), _, _, Some(cb), _, _) => {
                if let Some((_, normal)) = shapes::circles_are_intersecting(&ca.circle(), &pos_a, &cb.circle(), &pos_b) {
                    Self::add_collision(ent_a, ent_b, normal, data);
                }
            },
            (Some(ca), _, _, _, Some(bb), _)  => {
                if let Some(normal) = shapes::circle_plane_are_intersecting(&ca.circle(), &pos_a, &bb.boundary(), &pos_b) {
                    Self::add_collision(ent_a, ent_b, normal, data);
                }
            },
            (Some(_ca), _, _, _, _, Some(_pb)) => {
                // TODO
                // First test intersection of bounding boxes.
            },
            (_, Some(ba), _, Some(cb), _, _)  => {
                if let Some(normal) = shapes::circle_plane_are_intersecting(&cb.circle(), &pos_a, &ba.boundary(), &pos_b) {
                    Self::add_collision(ent_b, ent_a, normal, data);
                }
            },
            _ => (),
        }

    }

    ///
    fn detect_all_boundary_collisions(qt: &QuadTree<Index>, boundaries: &Vec<Entity>, data: &mut CollisionDetectionSysData) {

        for bnd_ent in boundaries {
            let plane = data.boundary_collider.get(*bnd_ent).unwrap().plane;
            let normal = plane.normal();
            let plane_pos = data.position.get(*bnd_ent).unwrap().vector;

            for ent_idx in qt.query_by_plane(&plane, &plane_pos) {
                let ent = data.entities.entity(ent_idx); // TODO Check is_alive()
                let candidate_pos = data.position.get(ent).unwrap().vector;

                if let Some(c) = data.circle_collider.get(ent) {
                    if let Some(_) = shapes::circle_plane_are_intersecting(c.circle(), &candidate_pos, &plane, &plane_pos) {
                        Self::add_collision(*bnd_ent, ent, *normal, data);
                    }
                }
            }
        }
    }

    /// Creates a new collision entity within the ECS, It details the two colliding entities and
    /// the collision normal vector from entity a to entity b.
    fn add_collision(ent_a: Entity, ent_b: Entity, normal: Vector2<f64>, data: &mut CollisionDetectionSysData) {
        let entity = data.entities.create();
        // unwrap cannot fail because entity has just been created.
        data.collision.insert(entity, Collision { ent_a, ent_b, normal }).unwrap();
    }

}

impl<'a> System<'a> for CollisionDetectionSys {
    type SystemData = CollisionDetectionSysData<'a>;

    // TODO possible extension: parallel join with rayon
    fn run(&mut self, mut data: Self::SystemData) {
        // create quadtree that covers the available screen area.
        let centre = vector![constants::FSCREEN_CENTRE_X, constants::FSCREEN_CENTRE_Y];
        let bounding_box = Aabb::new(constants::FSCREEN_EXTENT_X, constants::FSCREEN_EXTENT_Y);

        let mut qt = QuadTree::new();
        qt.initialize(centre, bounding_box);

        // insert the ids of all entities that have shape, position and mass into the tree.
        for (coll, pos, entity, _) in (&data.circle_collider, &data.position, &data.entities, &data.mass).join() {
            qt.insert(*coll.circle(), pos.vector, entity.id());
        }

        // walk tree and find/resolve collisions
        self.detect_all_internal_collisions(&qt, &mut data);

        // get a list of all boundary planes enclosing the space.
        let mut boundaries = vec![];
        for (ent, _) in (&data.entities, &data.boundary_collider).join() {
            boundaries.push(ent);
        }

        // find any collisions that occur between thses boundary planes and internal collidable
        // objects stored in the quadtree.
        Self::detect_all_boundary_collisions(&qt, &boundaries, &mut data);
    }
}
