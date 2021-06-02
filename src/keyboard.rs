use specs::prelude::*;

use crate::{
    components::*,
    constants,
    direction::Direction,
    resources::{MovementCommand, MovementCommandStack},
};

pub struct Keyboard;

#[derive(SystemData)]
pub struct KeyboardData<'a> {
    keyboard_controlled: ReadStorage<'a, KeyboardControlled>,
    movement_command_stack: WriteExpect<'a, MovementCommandStack>,
    propulsion: WriteStorage<'a, Propulsion>,
}

impl<'a> System<'a> for Keyboard {
    type SystemData = KeyboardData<'a>;

    // TODO possible extension: parallel join with rayon
    fn run(&mut self, mut data: Self::SystemData) {
        let movement_command = data.movement_command_stack.get_next();

        for (_, prop) in (&data.keyboard_controlled, &mut data.propulsion).join() {
            match movement_command {
                MovementCommand::Move(dir) => {
                    let (new_prop_x, new_prop_y) = match dir {
                        Direction::Up => (0.0, -constants::PLAYER_PROPULSION_FORCE),
                        Direction::Down => (0.0, constants::PLAYER_PROPULSION_FORCE),
                        Direction::Left => (-constants::PLAYER_PROPULSION_FORCE, 0.0),
                        Direction::Right => (constants::PLAYER_PROPULSION_FORCE, 0.0),
                    };

                    prop.x = new_prop_x;
                    prop.y = new_prop_y;
                },
                MovementCommand::Stop => {
                    prop.x = 0.0;
                    prop.y = 0.0;
                },
            }
        }
    }
}
