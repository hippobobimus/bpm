use bevy::{
    prelude::*,
    app::AppExit,
    math::DVec3,
};
use lazy_static::lazy_static;

use std::{
    collections::HashMap,
};

use crate::{
    physics::components::Thrust,
    user_interaction::components::{
        KeyboardControlled,
        Player,
    },
};

/// A SystemSet that handles keyboard interaction.
pub fn get_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(movement.system())
        .with_system(exit.system())
}

// A mapping that associates keycodes to the cardinal movement directions in which thrust will be
// applied.
lazy_static! {
    static ref MOVEMENT_KEYS_MAP: HashMap<KeyCode, DVec3> = {
        let mut map = HashMap::new();
        map.insert(KeyCode::Left, -DVec3::X);
        map.insert(KeyCode::Right, DVec3::X);
        map.insert(KeyCode::Up, -DVec3::Z);
        map.insert(KeyCode::Down, DVec3::Z);
        map.insert(KeyCode::Space, DVec3::Y);
        map.insert(KeyCode::H, -DVec3::X);
        map.insert(KeyCode::L, DVec3::X);
        map.insert(KeyCode::K, -DVec3::Z);
        map.insert(KeyCode::J, DVec3::Z);
        map.insert(KeyCode::W, DVec3::Y);
        map.insert(KeyCode::S, -DVec3::Y);
        map
    };
}

/// A system that manages keyboard interaction that applies thrust to an entity.
fn movement(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Thrust, (With<KeyboardControlled>, With<Player>)>,
) {
    for (key_code, dir) in MOVEMENT_KEYS_MAP.iter() {
        if keys.just_pressed(*key_code) {
            let mut thrust = query.single_mut().expect("There should only be one player!");

            thrust.engage(dir);
        }
        if keys.just_released(*key_code) {
            let mut thrust = query.single_mut().expect("There should only be one player!");

            thrust.disengage(dir);
        }
    }
}

/// A system that manages keyboard interactions that trigger an exit event.
fn exit(keys: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
