use nalgebra::{
    Vector2,
};
use specs::{
    prelude::*,
};

use crate::{
    components::*,
};

#[derive(SystemData)]
pub struct CollisionResponseSysData<'a> {
    collision: WriteStorage<'a, Collision>,
    player: ReadStorage<'a, Player>,
    render_colour: WriteStorage<'a, RenderColour>,
    entities: Entities<'a>,
    mass: ReadStorage<'a, Mass>,
    //updater: Read<'a, LazyUpdate>,
    velocity: WriteStorage<'a, Velocity>,
}

pub struct CollisionResponseSys;

impl CollisionResponseSys {

    ///
    fn calc_impulse(vel_a: &Velocity, vel_b: &Velocity, mass_a: &Mass, mass_b: &Mass, n: &Vector2<f64>) -> f64 {
        let vel_rel = vel_a.vector - vel_b.vector;

        let e = 1.0; // coefficient of restitution.

        let mass_eff = mass_a.inverse + mass_b.inverse;

        let impulse = -(1.0 + e) * vel_rel.dot(&n) / mass_eff;

        println!("IMPULSE={}", impulse);
        impulse
    }
}

impl<'a> System<'a> for CollisionResponseSys {
    type SystemData = CollisionResponseSysData<'a>;

    // TODO possible extension: parallel join with rayon
    fn run(&mut self, mut data: Self::SystemData) {

        for (coll_ent, collision) in (&data.entities, &data.collision).join() {
            let vel_a = data.velocity.get(collision.ent_a).unwrap();
            let vel_b = data.velocity.get(collision.ent_b).unwrap();

            let mass_a = data.mass.get(collision.ent_a).unwrap();
            let mass_b = data.mass.get(collision.ent_b).unwrap();

            let impulse = Self::calc_impulse(vel_a, vel_b, mass_a, mass_b, &collision.normal);

            let vel_a_mut = data.velocity.get_mut(collision.ent_a).unwrap();
            vel_a_mut.vector = vel_a_mut.vector + mass_a.inverse * impulse * collision.normal;

            let vel_b_mut = data.velocity.get_mut(collision.ent_b).unwrap();
            vel_b_mut.vector = vel_b_mut.vector - mass_b.inverse * impulse * collision.normal;

            // Change colour to red.
            if let Some(col_a) = data.render_colour.get_mut(collision.ent_a) {
                if data.player.get(collision.ent_a).is_none() {
                    col_a.change_colour(255, 0, 0);
                }
            }
            if let Some(col_b) = data.render_colour.get_mut(collision.ent_b) {
                if data.player.get(collision.ent_b).is_none() {
                    col_b.change_colour(255, 0, 0);
                }
            }


            // remove collision entity from ecs.
            let _ = data.entities.delete(coll_ent);
        }
    }
}
