use nalgebra::base::Vector2;
use specs::prelude::*;

//use std::time::SystemTime;

use crate::{
    components::*,
    constants,
    resources::*,
};

#[derive(SystemData)]
pub struct PhysicsData<'a> {
    mass: ReadStorage<'a, Mass>,
    position: WriteStorage<'a, Position>,
    aabb: WriteStorage<'a, AABB>,
    velocity: WriteStorage<'a, Velocity>,
    forces: WriteStorage<'a, Forces>,
    //propulsion: ReadStorage<'a, Propulsion>,
    //resistance: WriteStorage<'a, Resistance>,
    delta_time: ReadExpect<'a, DeltaTime>,
}

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = PhysicsData<'a>;

    // TODO possible extension: parallel join with rayon
    fn run(&mut self, mut data: Self::SystemData) {
        let dt_secs = data.delta_time.get_dt().as_secs_f64();

        for (mass, pos, vel, aabb, f) in (&data.mass, &mut data.position, &mut data.velocity,
                                          &mut data.aabb, &mut data.forces).join() {
            // update velocity based on current forces applied.
            Self::update_velocity(vel, mass, f, dt_secs);

            // update resistive force based on new velocity.
            //Self::update_resistance(&mut f.drag, vel);

            // attenuate any impact forces
            //Self::attenuate_impact(&mut f.impact);

            // update position based on new velocity.
            Self::update_position(pos, aabb, vel, dt_secs);
        }
//        for (mass, pos, vel, prop, res) in (&data.mass, &mut data.position, &mut data.velocity,
//                                            &data.propulsion, &mut data.resistance).join() {
//            // update velocity based on current forces applied.
//            Self::update_velocity(vel, mass, prop, res, dt_secs);
//
//            // update resistive force based on new velocity.
//            Self::update_resistance(res, vel);
//
//            // update position based on new velocity.
//            Self::update_position(pos, vel, dt_secs);
//        }
    }
}

impl Physics {
    fn update_velocity(vel: &mut Velocity, mass: &Mass, f: &Forces, dt: f64) {
        let transform = (f.propulsion + f.impact + f.drag) * mass.inverse * dt;

        vel.vel += transform;

        // ensure body comes to rest once the low velocity threshold has been reached.
        //if (f.propulsion.x == 0.0) && (f.impact.x == 0.0) && (vel.x.abs() < constants::LOW_VELOCITY_THRESHOLD) { vel.x = 0.0 };
        //if (f.propulsion.y == 0.0) && (vel.y.abs() < constants::LOW_VELOCITY_THRESHOLD) { vel.y = 0.0 };
    }

    fn update_resistance(drag: &mut Vector2<f64>, vel: &Velocity) {
        let v_mag = vel.vel.magnitude();
        let v_unit = vel.vel.normalize();

        *drag = -1.0 * constants::RESISTANCE_COEFFICIENT * v_mag * v_mag * v_unit;
        //drag.x = constants::RESISTANCE_COEFFICIENT * vel.x * vel.x.abs();
        //drag.y = constants::RESISTANCE_COEFFICIENT * vel.y * vel.y.abs();
    }

    fn update_position(pos: &mut Position, aabb: &mut AABB, vel: &Velocity, dt: f64) {
        //pos.x = pos.x + vel.x * dt;
        //pos.y = pos.y + vel.y * dt;

        let transform = vel.vel * dt;

        pos.pos += transform;

        aabb.min += transform;
        aabb.max += transform;
    }

//    fn attenuate_impact(impact: &mut Force) {
//        impact.x *= 0.95;
//        impact.y *= 0.95;
//        if impact.x.abs() < 1.0 {
//            impact.x = 0.0;
//        }
//        if impact.y.abs() < 1.0 {
//            impact.y = 0.0;
//        }
//    }
}
//impl Physics {
//    fn update_velocity(vel: &mut Velocity, mass: &Mass, prop: &Propulsion, res: &Resistance,
//                       dt: f64) {
//        vel.x = vel.x + ((prop.x - res.x) / mass.value) * dt;
//        vel.y = vel.y + ((prop.y - res.y) / mass.value) * dt;
//
//        // ensure body comes to rest once the low velocity threshold has been reached.
//        if (prop.x == 0.0) && (vel.x.abs() < constants::LOW_VELOCITY_THRESHOLD) { vel.x = 0.0 };
//        if (prop.y == 0.0) && (vel.y.abs() < constants::LOW_VELOCITY_THRESHOLD) { vel.y = 0.0 };
//    }
//
//    fn update_resistance(res: &mut Resistance, vel: &Velocity) {
//        res.x = constants::RESISTANCE_COEFFICIENT * vel.x * vel.x.abs();
//        res.y = constants::RESISTANCE_COEFFICIENT * vel.y * vel.y.abs();
//    }
//
//    fn update_position(pos: &mut Position, vel: &Velocity, dt: f64) {
//        pos.x = pos.x + vel.x * dt;
//        pos.y = pos.y + vel.y * dt;
//    }
//}

//#[derive(SystemData)]
//pub struct CollisionDetectionData<'a> {
//    player_char: ReadStorage<'a, PlayerCharacter>,
//    hit_box: ReadStorage<'a, AABB>,
//    position: ReadStorage<'a, Position>,
//    forces: WriteStorage<'a, Forces>,
//}
//
//pub struct CollisionDetection;
//
//impl<'a> System<'a> for CollisionDetection {
//    type SystemData = CollisionDetectionData<'a>;
//
//    fn run(&mut self, mut data: Self::SystemData) {
//        for (_, aabb, pos, f) in (&data.player_char, &data.hit_box, &data.position, &mut data.forces).join() {
//            for ((), aabb_other, pos_other) in (!&data.player_char, &data.hit_box, &data.position).join() {
//
//                // ignore same entity
//
//                let min_x_subject = pos.x;
//                let min_y_subject = pos.y;
//                let max_x_subject = pos.x + aabb.width;
//                let max_y_subject = pos.y + aabb.height;
//
//                let min_x_object = pos_other.x;
//                let min_y_object = pos_other.y;
//                let max_x_object = pos_other.x + aabb_other.width;
//                let max_y_object = pos_other.y + aabb_other.height;
//
//                // projections
//                let d1x = min_x_object - max_x_subject;
//                let d1y = min_y_object - max_y_subject;
//                let d2x = min_x_subject - max_x_object;
//                let d2y = min_y_subject - max_y_object;
//
//                let mut colliding = true;
//                // check for overlapping
//                if (d1x > 0.0) || (d1y > 0.0) {
//                    colliding = false;
//                }
//                if (d2x > 0.0) || (d2y > 0.0) {
//                    colliding = false;
//                }
//
//                // collision resolution
//                if colliding {
//                    println!("COLLIDING {:?}", SystemTime::now());
//                    f.impact.x = -100.0;
//                    f.impact.y = 0.0;
//                }
//            }
//        }
//    }
//}
