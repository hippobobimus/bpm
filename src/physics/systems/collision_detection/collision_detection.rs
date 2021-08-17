use bevy::{
    prelude::*,
    math::DVec3,
};

use crate::{
    constants,
    physics::collision_detection::{
        self,
        Contact,
    },
    physics::components::{
        BoundaryCollider,
        Collider,
        Mass,
        PhysTransform,
    },
    physics::shapes::Aabb3D,
    physics::oct_tree::{
        OctIndex,
        OctTree,
        OctTreeNode,
    },
};

/// A vector list containing possible collisions represented by the pair of Entitys concerned.
type CollisionCandidates = Vec<(Entity, Entity)>;

/// A SystemSet covering collision detection and contact generation processes.
pub fn get_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(update_tree.system()
                     .label("tree update")
        )
        .with_system(broad_phase.system()
                     .label("broad phase")
                     .after("tree update")
        )
        .with_system(contact_generation.system()
                     .after("broad phase")
        )
}

/// A system to be run at startup that performs necessary setup for collision detection to run.
///
/// Creates resources required for collision detection and contact generation. Namely, an OctTree
/// used for spatial partitioning, filling it with currently available primative shapes, and the
/// CollisionCandidates vector.
pub fn initialize(
    mut commands: Commands,
    shapes_query: Query<(Entity, &Collider, &PhysTransform), With<Mass>>,
) {
    // create OctTree that covers the gameplay volume.
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

    // Create collision candidates resource.
    let collision_candidates: CollisionCandidates = vec![];
    commands.insert_resource(collision_candidates);
}

/// Updates the OctTree by inserting any new entities with a Collider into the tree and updating
/// the position of any entities that have moved since the last frame.
fn update_tree(
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


/// Broad phase collision detection that generates collision candidates by finding appropriate in
/// close proximity using the OctTree's spatial partitioning.
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

/// Narrow-phase collision detection and contact generation.
fn contact_generation(
    collider_query: Query<(&Collider, &PhysTransform)>,
    boundary_query: Query<(&BoundaryCollider, &PhysTransform)>,
    mut candidates: ResMut<CollisionCandidates>,
) {
    // work through the collision candidates list of primatives produced by the OctTree and generate
    // contacts.
    while let Some((ent_a, ent_b)) = candidates.pop() {
        if let (Ok((collider_a, transform_a)), Ok((collider_b, transform_b))) =
            (collider_query.get(ent_a), collider_query.get(ent_b))
        {
            let contacts = collision_detection::generate_primative_contacts(
                &collider_a.0,
                &collider_b.0,
                transform_a,
                transform_b,
            );

            if let Some(c) = contacts {
                process_contacts(c);
            }
        }
    }

    // test all internal colliders for contact with the boundaries.
    for (bnd, bnd_transform) in boundary_query.iter() {
        for (coll, coll_transform) in collider_query.iter() {
            let contacts = collision_detection::generate_boundary_contacts(
                &bnd.0,
                &coll.0,
                bnd_transform,
                coll_transform,
            );

            if let Some(c) = contacts {
                process_contacts(c);
            }
        }
    }
}

// TODO
fn process_contacts(contacts: Vec<Contact>) {
    println!("{:?}", contacts);
}
