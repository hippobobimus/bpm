use specs::prelude::*;

use crate::{
    components::*,
    constants,
    resources::*,
};

#[derive(SystemData)]
pub struct IntegrationSysData<'a> {
    force: ReadStorage<'a, Force>,
    mass: ReadStorage<'a, Mass>,
    position: WriteStorage<'a, Position>,
    velocity: WriteStorage<'a, Velocity>,
    delta_time: ReadExpect<'a, DeltaTime>,
}

pub struct IntegrationSys;

impl<'a> System<'a> for IntegrationSys {
    type SystemData = IntegrationSysData<'a>;

    // TODO possible extension: parallel join with rayon
    fn run(&mut self, mut data: Self::SystemData) {
        let dt_secs = data.delta_time.get_dt().as_secs_f64();

        for (f, m, p, v) in (&data.force, &data.mass, &mut data.position, &mut data.velocity).join() {
            // Infinite mass objects cannot move.
            if m.is_infinite() { break };

            // Update position.
            p.transform(&(v.vector() * dt_secs));

            // Calculate acceleration from currently applied forces.
            let a = f.vector() * m.inverse();

            // Update velocity.
            v.transform(&(a * dt_secs));

            // Apply damping.
            v.scale(constants::DAMPING_FACTOR.powf(dt_secs));

            // TODO reset forces here?
        }
    }
}
