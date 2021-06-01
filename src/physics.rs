use specs::prelude::*;

use crate::{
    components::*,
    constants,
};

#[derive(SystemData)]
pub struct PhysicsData<'a> {
    mass: ReadStorage<'a, Mass>,
    position: WriteStorage<'a, Position>,
    velocity: WriteStorage<'a, Velocity>,
    propulsion: ReadStorage<'a, Propulsion>,
    resistance: WriteStorage<'a, Resistance>,
}

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = PhysicsData<'a>;

    // TODO possible extension: parallel join with rayon
    fn run(&mut self, mut data: Self::SystemData) {
        for (mass, pos, vel, prop, res) in (&data.mass, &mut data.position, &mut data.velocity,
                                            &data.propulsion, &mut data.resistance).join() {
            let dt = 1.0 / 20.0; // 20 Hz

            // update velocity based on current forces applied.
            Self::update_velocity(vel, mass, prop, res, dt);

            // update resistive force based on new velocity.
            Self::update_resistance(res, vel);

            // update position based on new velocity.
            Self::update_position(pos, vel, dt);
        }
    }

}

impl Physics {

    fn update_velocity(vel: &mut Velocity, mass: &Mass, prop: &Propulsion, res: &Resistance,
                       dt: f64) {
        vel.x = vel.x + ((prop.x - res.x) / mass.value) * dt;
        vel.y = vel.y + ((prop.y - res.y) / mass.value) * dt;

        // ensure body comes to rest once the low velocity threshold has been reached.
        if vel.x.abs() < 5.0 { vel.x = 0.0 };
        if vel.y.abs() < 5.0 { vel.y = 0.0 };
    }

    fn update_resistance(res: &mut Resistance, vel: &Velocity) {
        res.x = constants::RESISTANCE_COEFFICIENT * vel.x * vel.x.abs();
        res.y = constants::RESISTANCE_COEFFICIENT * vel.y * vel.y.abs();
    }

    fn update_position(pos: &mut Position, vel: &Velocity, dt: f64) {
        pos.x = pos.x + vel.x * dt;
        pos.y = pos.y + vel.y * dt;
    }
}
