use specs::prelude::*;

use crate::{
    components::*,
    resources::{MovementCommand, MovementCommandStack},
};

pub struct Keyboard;

#[derive(SystemData)]
pub struct KeyboardData<'a> {
    keyboard_controlled: ReadStorage<'a, KeyboardControlled>,
    movement_command_stack: WriteExpect<'a, MovementCommandStack>,
    thrust: WriteStorage<'a, Thrust>,
}

impl<'a> System<'a> for Keyboard {
    type SystemData = KeyboardData<'a>;

    // TODO possible extension: parallel join with rayon
    fn run(&mut self, mut data: Self::SystemData) {
        let movement_command = data.movement_command_stack.get_next();

        for (_, thrust) in (&data.keyboard_controlled, &mut data.thrust).join() {
            match movement_command {
                MovementCommand::Move(dir) => {
                    //f.propulsion = dir.unit_vector() * constants::PLAYER_PROPULSION_FORCE;
                    thrust.engage(&dir);
                },
                MovementCommand::Stop => {
                    //f.propulsion = vector![0.0, 0.0];
                    thrust.disengage();
                },
            }
        }
    }
}
