mod components;
mod systems;

/// 'use debug::prelude::*;' to import common components, bundles and plugins.
pub mod prelude {
    pub use super::BpmDebugPlugin;
}

use bevy::prelude::*;

/// A plugin that overlays onto the screen basic debugging information related to the physics
/// engine.
pub struct BpmDebugPlugin;

impl Plugin for BpmDebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(
                systems::initialize.system()
            )
            .add_system_set(
                systems::get_system_set()
            );
    }
}
