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
    components::*,
    physics::Thrust,
};

// A mapping that associates keycodes to the cardinal movement directions.
lazy_static! {
    static ref MOVEMENT_KEYS_MAP: HashMap<KeyCode, DVec3> = {
        let mut map = HashMap::new();
        map.insert(KeyCode::Left, -DVec3::X);
        map.insert(KeyCode::Right, DVec3::X);
        map.insert(KeyCode::Up, -DVec3::Z);
        map.insert(KeyCode::Down, DVec3::Z);
        map.insert(KeyCode::Space, DVec3::Y);
        map
    };
}

// Plugins

pub struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(movement.system())
            .add_system(exit.system());
    }
}

// Systems

fn movement(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&KeyboardControlled, &Player, &mut Thrust)>,
) {
    for (key_code, dir) in MOVEMENT_KEYS_MAP.iter() {
        if keys.just_pressed(*key_code) {
            let (_kb, _player, mut thrust) = query.single_mut()
                .expect("There should only be one player!");

            thrust.engage(dir);
        } else if keys.just_released(*key_code) {
            let (_kb, _player, mut thrust) = query.single_mut()
                .expect("There should only be one player!");

            thrust.disengage(dir);
        }
    }
}

fn exit(keys: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
