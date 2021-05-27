use specs::prelude::*;

use crate::{
    components::*,
    direction::Direction,
};

#[derive(SystemData)]
pub struct PhysicsData<'a> {
    position: WriteStorage<'a, Position>,
    velocity: ReadStorage<'a, Velocity>,
}

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = PhysicsData<'a>;

    // TODO possible extension: parallel join with rayon
    fn run(&mut self, mut data: Self::SystemData) {
        for (pos, vel) in (&mut data.position, &data.velocity).join() {
            match vel.direction {
                Direction::Left => {
                    pos.0 = pos.0.offset(-vel.speed, 0);
                },
                Direction::Right => {
                    pos.0 = pos.0.offset(vel.speed, 0);
                },
                Direction::Up => {
                    pos.0 = pos.0.offset(0, -vel.speed);
                },
                Direction::Down => {
                    pos.0 = pos.0.offset(0, vel.speed);
                },
            }
        }
    }

}
