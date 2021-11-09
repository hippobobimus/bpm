use bevy::prelude::*;

use crate::{
    physics::components::{
        PhysTransform,
    },
};

/// System labels covering sub-systems in the synchronisation process.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum SyncSystems {
    SyncTransforms,
}
/// A SystemSet that performs syncronisation between 64-bit Bpm components and their 32-bit Bevy
/// counterparts.
pub fn get_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(sync_transforms.system()
                     .label(SyncSystems::SyncTransforms)
        )
}

/// Updates the 32-bit Bevy Transform from the 64-bit Bpm PhysTransform.
fn sync_transforms(
    //mut transforms: Query<(&PhysTransform, &mut Transform), Changed<PhysTransform>>,
    mut transforms: Query<(&PhysTransform, &mut Transform)>,
) {
    for (phys_transform, mut transform) in transforms.iter_mut() {
        transform.translation = phys_transform.translation.as_f32();
        transform.rotation = phys_transform.rotation.as_f32();
    }
}
