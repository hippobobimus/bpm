use bevy::{
    prelude::*,
    math::DVec3,
};

use crate::{
    constants,
    physics::collision_detection,
    physics::components::{BoundaryCollider, Collider, Mass, PhysTransform},
    physics::shapes::Aabb3D,
    physics::oct_tree::{OctIndex, OctTree, OctTreeNode},
};

pub fn initialize(
    mut commands: Commands,
    shapes_query: Query<(Entity, &Collider, &PhysTransform), With<Mass>>,
) {
    // create oct-tree that covers the gameplay volume.
    let centre = DVec3::new(
        constants::PLAY_AREA_CENTRE_X,
        constants::PLAY_AREA_CENTRE_Y,
        constants::PLAY_AREA_CENTRE_Y,
    );
    let bounding_box = Aabb3D::from_xyz(
        constants::PLAY_AREA_EXTENT_X,
        constants::PLAY_AREA_EXTENT_Y,
        constants::PLAY_AREA_EXTENT_Y,
    );

    let mut tree = OctTree::new(constants::MAX_OCT_TREE_DEPTH);
    tree.initialize(centre, bounding_box);

    // insert the ids of all entities (with mass) that have a primative shape for collisions.
    for (ent, collider, transform) in shapes_query.iter() {
        tree.insert(collider, transform, ent);
    }

    commands.insert_resource(tree);

    let collision_candidates: CollisionCandidates = vec![];
    commands.insert_resource(collision_candidates);
}

pub fn update_tree(
    added_query: Query<(Entity, &Collider, &PhysTransform), (With<Mass>, Added<PhysTransform>)>,
    moved_query: Query<(Entity, &Collider, &PhysTransform), (With<Mass>, Changed<PhysTransform>)>,
    mut tree: ResMut<OctTree<Entity>>,
) {
    // insert any new items into the tree.
    for (ent, collider, transform) in added_query.iter() {
        tree.insert(collider, transform, ent);
    }

    // update any existing items that have moved.
    for (ent, collider, transform) in moved_query.iter() {
        tree.update(collider, transform, ent);
    }
}

pub type CollisionCandidates = Vec<(Entity, Entity)>;

/// Broad phase collision detection that generates collision candidates.
pub fn broad_phase(
    tree: Res<OctTree<Entity>>,
    mut candidates: ResMut<CollisionCandidates>,
) {
    fn helper(
        tree: &OctTree<Entity>,
        node: &OctTreeNode<Entity>,
        ancestor_stack: &mut Vec<OctIndex>,
        candidates: &mut ResMut<CollisionCandidates>,
    ) {
        // add current node to ancestor stack
        ancestor_stack.push(node.get_idx());

        for ancestor_idx in ancestor_stack.iter() {
            let ancestor_node = tree.get_node(*ancestor_idx)
                .expect("Node indexed in ancestor stack not found!");
            let ancestor_ents = ancestor_node.get_data();

            for ent_a in ancestor_ents {
                let current_ents = node.get_data();

                for ent_b in current_ents {
                    // ignore self on self collisions and duplicates (i.e. 1->2 and 2->1).
                    if ent_a == ent_b { break; }

                    candidates.push((*ent_a, *ent_b));
                }
            }
        }

        // preorder tree traversal.
        for c in node.get_child_indices().iter() {
            if let Some(child_idx) = c {
                let child_node = tree.get_node(*child_idx).unwrap();
                helper(tree, child_node, ancestor_stack, candidates);
            }
        }

        // remove current node from ancestor stack
        ancestor_stack.pop();
    }

    let root_node = tree.get_root_node().expect("The OctTree has not been initialised!");
    let mut stack = vec![];

    helper(&*tree, root_node, &mut stack, &mut candidates);
}

pub fn contact_generation(
    primatives_query: Query<(&Collider, &PhysTransform)>,
    boundaries_query: Query<(&BoundaryCollider, &PhysTransform)>,
    mut candidates: ResMut<CollisionCandidates>,
) {
    while let Some((ent_a, ent_b)) = candidates.pop() {
        if let (Ok((collider_a, transform_a)), Ok((collider_b, transform_b))) =
            (primatives_query.get(ent_a), primatives_query.get(ent_b))
        {
            let contacts = collision_detection::generate_primative_contacts(
                &collider_a.0,
                &collider_b.0,
                transform_a,
                transform_b,
            );

            if let Some(c) = contacts {
                println!("{:?}", c);
            }
        }
    }

    // TODO boundary contacts...
}
//    ///
//    fn detect_all_boundary_collisions(qt: &QuadTree<Index>, boundaries: &Vec<Entity>, data: &mut CollisionDetectionSysData) {
//
//        for bnd_ent in boundaries {
//            let plane = data.boundary_collider.get(*bnd_ent).unwrap().plane;
//            let normal = plane.normal();
//            let plane_pos = data.position.get(*bnd_ent).unwrap().vector;
//
//            for ent_idx in qt.query_by_plane(&plane, &plane_pos) {
//                let ent = data.entities.entity(ent_idx); // TODO Check is_alive()
//                let candidate_pos = data.position.get(ent).unwrap().vector;
//
//                if let Some(c) = data.circle_collider.get(ent) {
//                    if let Some(_) = shapes::circle_plane_are_intersecting(c.circle(), &candidate_pos, &plane, &plane_pos) {
//                        Self::add_collision(*bnd_ent, ent, *normal, data);
//                    }
//                }
//            }
//        }
//    }
//
//    /// Creates a new collision entity within the ECS, It details the two colliding entities and
//    /// the collision normal vector from entity a to entity b.
//    fn add_collision(ent_a: Entity, ent_b: Entity, normal: Vector2<f64>, data: &mut CollisionDetectionSysData) {
//        let entity = data.entities.create();
//        // unwrap cannot fail because entity has just been created.
//        data.collision.insert(entity, Collision { ent_a, ent_b, normal }).unwrap();
//    }
//
//}
//
