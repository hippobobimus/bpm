use bevy::prelude::*;

use crate::{
    physics::components::{
        BoundaryCollider,
        InertiaTensor,
        PhysTransform,
    },
};

/// System labels covering sub-systems in the cache update process.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum CacheUpdateSystems {
    Update,
}

/// A SystemSet that updates any cached data that relies on the current transform.
pub fn get_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(update_cached_data.system()
                     .label(CacheUpdateSystems::Update)
        )
}

/// Updates any cached derived data that relies on the PhysTransform, for Entitys that have moved.
fn update_cached_data(
    mut set: QuerySet<(
        Query<(&mut PhysTransform, &mut InertiaTensor), Changed<PhysTransform>>,
        Query<(&PhysTransform, &mut BoundaryCollider), Changed<PhysTransform>>,
    )>,
) {
    for (mut transform, mut inertia_tensor) in set.q0_mut().iter_mut() {
        transform.update();
        inertia_tensor.update(transform.matrix());
    }
    for (transform, mut boundary) in set.q1_mut().iter_mut() {
        // update the cached plane normal in the boundary if the boundary has moved.
        boundary.0.update(transform);
    }
}
