use specs::prelude::*;

use crate::{
    commands::MovementCommand,
    components::*,
    constants,
    //direction::Direction,
};

pub struct Keyboard;

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        ReadExpect<'a, Option<MovementCommand>>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Velocity>,
    );

    // possible extension: parallel join with rayon
    fn run(&mut self, mut data: Self::SystemData) {
        // TODO irrefutable patterns
        let movement_command = match &*data.0 {
            Some(movement_command) => movement_command,
            None => return,
        };

        for (_, vel) in (&data.1, &mut data.2).join() {
            match movement_command {
                &MovementCommand::Move(direction) => {
                    vel.speed = constants::PLAYER_MOVEMENT_SPEED;
                    vel.direction = direction;
                },
                MovementCommand::Stop => vel.speed = 0,
            }
        }
    }
}
