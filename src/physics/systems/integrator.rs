use bevy::prelude::*;

use crate::{
    constants,
    physics::{Force, Mass, Velocity},
};

pub fn integrator(
    time: Res<Time>,
    mut query: Query<(&Force, &Mass, &mut Transform, &mut Velocity)>,
) {
    let dt_secs = time.delta_seconds_f64();

    for (f, m, mut p, mut v) in query.iter_mut() {
        // Infinite mass objects cannot move.
        if m.is_infinite() { break };

        // Update position.
        // TODO deal with f32 f64 precision conflict in position
        let translation = *v.vector() * dt_secs;
        p.translation += Vec3::new(translation.x as f32,
                                   translation.y as f32,
                                   translation.z as f32);

        // Calculate acceleration from currently applied forces.
        let a = *f.vector() * m.inverse();

        // Update velocity.
        v.add(a * dt_secs);

        // Apply damping.
        v.scale(constants::DAMPING_FACTOR.powf(dt_secs));

        // If velocity is very low, make it 0
        if v.vector().length_squared() < constants::LOW_VELOCITY_THRESHOLD {
            v.zero();
        }

        // TODO reset forces here?
    }
}

//#[derive(SystemData)]
//pub struct IntegrationSysData<'a> {
//    force: ReadStorage<'a, Force>,
//    mass: ReadStorage<'a, Mass>,
//    position: WriteStorage<'a, Position>,
//    velocity: WriteStorage<'a, Velocity>,
//    delta_time: ReadExpect<'a, DeltaTime>,
//}
//
//pub struct IntegrationSys;
//
//impl<'a> System<'a> for IntegrationSys {
//    type SystemData = IntegrationSysData<'a>;
//
//    // TODO possible extension: parallel join with rayon
//    fn run(&mut self, mut data: Self::SystemData) {
//        let dt_secs = data.delta_time.get_dt().as_secs_f64();
//
//        for (f, m, p, v) in (&data.force, &data.mass, &mut data.position, &mut data.velocity).join() {
//            // Infinite mass objects cannot move.
//            if m.is_infinite() { break };
//
//            // Update position.
//            p.transform(&(v.vector() * dt_secs));
//
//            // Calculate acceleration from currently applied forces.
//            let a = f.vector() * m.inverse();
//
//            // Update velocity.
//            v.transform(&(a * dt_secs));
//
//            // Apply damping.
//            v.scale(constants::DAMPING_FACTOR.powf(dt_secs));
//
//            // TODO reset forces here?
//        }
//    }
//}
