use specs::prelude::*;

use crate::{
    components::*,
    resources::*,
};

#[derive(SystemData)]
pub struct MovementSysData<'a> {
    position: WriteStorage<'a, Position>,
    velocity: WriteStorage<'a, Velocity>,
    delta_time: ReadExpect<'a, DeltaTime>,
}

pub struct MovementSys;

impl<'a> System<'a> for MovementSys {
    type SystemData = MovementSysData<'a>;

    // TODO possible extension: parallel join with rayon
    fn run(&mut self, mut data: Self::SystemData) {
        let dt_secs = data.delta_time.get_dt().as_secs_f64();

        for (pos, vel) in (&mut data.position, &mut data.velocity).join() {
            // integrate position.
            pos.vector += vel.vector * dt_secs;
        }
    }
}
