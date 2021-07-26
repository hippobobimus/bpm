use bevy::prelude::*;

use crate::{
    physics::{Drag, Force, Gravity, Mass, Thrust, Velocity},
};

pub fn force_accumulation(
    mut q: QuerySet<(
        Query<&mut Force>,
        Query<(&Drag, &mut Force, &Velocity)>,
        Query<(&Gravity, &mut Force, &Mass)>,
        Query<(&Thrust, &mut Force)>,
    )>,
) {
    // Zero currently applied forces.
    for mut f in q.q0_mut().iter_mut() {
        f.reset();
    }

    // Apply force generators.
    for (drag, mut f, v) in q.q1_mut().iter_mut() {
        f.add(drag.force(*v.vector()));
    }
    for (gravity, mut f, m) in q.q2_mut().iter_mut() {
        // ensure the mass is not 0 or infinite (or subnormal/NaN).
        if !m.is_normal() { break };
        f.add(gravity.force(m.value()));
    }
    for (thrust, mut f) in q.q3_mut().iter_mut() {
        f.add(*thrust.force());
    }
}

//#[derive(SystemData)]
//pub struct ForceSysData<'a> {
//    drag: ReadStorage<'a, Drag>,
//    force: WriteStorage<'a, Force>,
//    gravity: ReadStorage<'a, Gravity>,
//    mass: ReadStorage<'a, Mass>,
//    thrust: ReadStorage<'a, Thrust>,
//    velocity: WriteStorage<'a, Velocity>,
//}
//
///// Updates the force accumulator of each entity with all force generators assigned to the entity.
//pub struct ForceSys;
//
//impl<'a> System<'a> for ForceSys {
//    type SystemData = ForceSysData<'a>;
//
//    // TODO possible extension: parallel join with rayon
//    fn run(&mut self, mut data: Self::SystemData) {
//        //let dt_secs = data.delta_time.get_dt().as_secs_f64();
//
//        // Zero currently applied forces.
//        for f in (&mut data.force).join() {
//            f.reset();
//        }
//
//        // Apply force generators.
//        for (drag, v, f) in (&data.drag, &data.velocity, &mut data.force).join() {
//            f.add_force(&drag.force(&v.vector));
//        }
//        for (gravity, m, f) in (&data.gravity, &data.mass, &mut data.force).join() {
//            // ensure the mass is not 0 or infinite (or subnormal/NaN).
//            if !m.is_normal() { break };
//            f.add_force(&gravity.force(m.value()));
//        }
//        for (thrust, f) in (&data.thrust, &mut data.force).join() {
//            f.add_force(&thrust.force());
//        }
//    }
//}
