pub mod components;
mod systems;

pub use components::{
    KeyboardControlled,
    Player,
};

use bevy::prelude::*;

use systems::keyboard;

/// A plugin that adds systems for handling user interaction. For example, with the keyboard.
pub struct UserInteractionPlugin;

impl Plugin for UserInteractionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(keyboard::get_system_set());
    }
}

/// 'use user_interaction::prelude::*;' to add common components and the plugin.
pub mod prelude {
    pub use super::components::{
        KeyboardControlled,
        Player,
    };
    pub use super::UserInteractionPlugin;
}
