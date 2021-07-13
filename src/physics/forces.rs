use specs::prelude::*;

use crate::{
    components::*,
};

#[derive(SystemData)]
pub struct ForceSysData<'a> {
    drag: ReadStorage<'a, Drag>,
    force: WriteStorage<'a, Force>,
    gravity: ReadStorage<'a, Gravity>,
    mass: ReadStorage<'a, Mass>,
    thrust: ReadStorage<'a, Thrust>,
    velocity: WriteStorage<'a, Velocity>,
}

/// Updates the force accumulator of each entity with all force generators assigned to the entity.
pub struct ForceSys;

impl<'a> System<'a> for ForceSys {
    type SystemData = ForceSysData<'a>;

    // TODO possible extension: parallel join with rayon
    fn run(&mut self, mut data: Self::SystemData) {
        //let dt_secs = data.delta_time.get_dt().as_secs_f64();

        // Zero currently applied forces.
        for f in (&mut data.force).join() {
            f.reset();
        }

        // Apply force generators.
        for (drag, v, f) in (&data.drag, &data.velocity, &mut data.force).join() {
            f.add_force(&drag.force(&v.vector));
        }
        for (gravity, m, f) in (&data.gravity, &data.mass, &mut data.force).join() {
            // ensure the mass is not 0 or infinite (or subnormal/NaN).
            if !m.is_normal() { break };
            f.add_force(&gravity.force(m.value()));
        }
        for (thrust, f) in (&data.thrust, &mut data.force).join() {
            f.add_force(&thrust.force());
        }
    }
}

//impl ForceSys {
//    // TODO Move to separate system
//    fn update_velocity(forces: &Forces, mass: &Mass, vel: &mut Velocity, dt: f64) {
//        let vel_transform = (forces.propulsion + forces.drag) * mass.inverse * dt;
//
//        vel.vector += vel_transform;
//
//        if vel.vector.x.is_nan() || vel.vector.y.is_nan() {
//            panic!("NAN");
//        }
//
//        // ensure body comes to rest once the low velocity threshold has been reached.
//        if (forces.propulsion.x == 0.0) && (vel.vector.x.abs() < constants::LOW_VELOCITY_THRESHOLD) {
//                vel.vector.x = 0.0;
//        };
//        if (forces.propulsion.y == 0.0) && (vel.vector.y.abs() < constants::LOW_VELOCITY_THRESHOLD) {
//            vel.vector.y = 0.0;
//        };
//    }
//
//    fn update_drag(forces: &mut Forces, vel: &Velocity) {
//        let vel_unit_dir = match vel.vector.try_normalize(0.01) {
//            Some(d) => d,
//            None => return,
//        };
//        let vel_mag = vel.vector.magnitude();
//
//        forces.drag = -1.0 * constants::RESISTANCE_COEFFICIENT * vel_mag * vel_mag * vel_unit_dir;
//    }
//}
//
