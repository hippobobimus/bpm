use specs::prelude::*;

use crate::{
    components::*,
    constants,
    resources::{MovementCommand, MovementCommandQueue},
};

pub struct Keyboard;

#[derive(SystemData)]
pub struct KeyboardData<'a> {
    movement_command_queue: WriteExpect<'a, MovementCommandQueue>,
    keyboard_controlled: ReadStorage<'a, KeyboardControlled>,
    velocity: WriteStorage<'a, Velocity>,
}

impl<'a> System<'a> for Keyboard {
    type SystemData = KeyboardData<'a>;

    // TODO possible extension: parallel join with rayon
    // irrefutable patterns
    fn run(&mut self, mut data: Self::SystemData) {
        let movement_command = data.movement_command_queue.get_next();

        for (_, vel) in (&data.keyboard_controlled, &mut data.velocity).join() {
            match movement_command {
                MovementCommand::Move(dir) => {
                    vel.speed = constants::PLAYER_MOVEMENT_SPEED;
                    vel.direction = dir;
                },
                MovementCommand::Stop => {
                    vel.speed = 0;
                },
            }
        }
    }
}
