use lazy_static::lazy_static;
use sdl2::{
    event::Event,
    EventPump,
    keyboard::Keycode,
};
use specs::prelude::*;

use std::{
    collections::HashMap,
};

use crate::{
    resources::MovementCommandStack,
    direction::Direction,
};

// A mapping that associates keycodes to the cardinal movement directions.
lazy_static! {
    static ref MOVEMENT_KEYS_MAP: HashMap<Keycode, Direction> = {
        let mut map = HashMap::new();
        map.insert(Keycode::Left, Direction::Left);
        map.insert(Keycode::Right, Direction::Right);
        map.insert(Keycode::Up, Direction::Up);
        map.insert(Keycode::Down, Direction::Down);
        map
    };
}

/// Returns true if the game loop should be exited.
pub fn process_events(world: &World, event_pump: &mut EventPump) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return true;
            },
            Event::KeyDown { keycode: Some(k), repeat: false, .. } if MOVEMENT_KEYS_MAP.contains_key(&k) => {
                println!("key down");
                let dir = MOVEMENT_KEYS_MAP.get(&k).unwrap(); // already verified contains

                let mut mcs = world.write_resource::<MovementCommandStack>();
                mcs.add(*dir);
            },
            Event::KeyUp { keycode: Some(k), repeat: false, .. } if MOVEMENT_KEYS_MAP.contains_key(&k) => {
                println!("key up");
                let dir = MOVEMENT_KEYS_MAP.get(&k).unwrap();

                let mut mcs = world.write_resource::<MovementCommandStack>();
                mcs.remove(*dir);
            },
            _ => {}
            }
    }

    false
}
