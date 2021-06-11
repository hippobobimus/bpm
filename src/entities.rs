use nalgebra::{
    vector,
};
use specs::prelude::*;

use crate::{
    components::*,
    constants,
};

pub struct Spawner<'a> {
    world: &'a mut World,
}

impl<'a> Spawner<'a> {
    pub fn new(world: &'a mut World) -> Self {
        Self { world }
    }

    pub fn spawn_player(&mut self, spritesheet: usize, mass: f64, x_pos: f64, y_pos: f64) {
        let player_animation = MovementAnimation::new(spritesheet,
                                                      *constants::SPRITESHEET_INITIAL_FRAME);
    
        self.world.create_entity()
                  //.with(PlayerCharacter)
                  .with(KeyboardControlled)
                  .with(Mass { value: mass, inverse: 1.0 / mass })
                  .with(Forces::new())
                  .with(Position { pos: vector![0.0, 0.0] })
                  .with(AABB { min: vector![0.0, 0.0], max: vector![12.0, 17.0] })
                  .with(Velocity { vel: vector![0.0, 0.0] })
                  .with(player_animation.down_frames[0].clone()) // Sprite
                  .with(player_animation)
                  .build();
    }

    pub fn spawn_npc(&mut self, spritesheet: usize, mass: f64, x_pos: f64, y_pos: f64) {
        let animation = MovementAnimation::new(spritesheet,
                                                      *constants::SPRITESHEET_INITIAL_FRAME);
    
        self.world.create_entity()
                  .with(Mass { value: mass, inverse: 0.0 })  // TODO inverse mass
                  .with(Forces::new())
                  .with(Position { pos: vector![x_pos, y_pos] })
                  //.with(Velocity { x: 0.0, y: 0.0 })
                  .with(AABB { min: vector![x_pos, y_pos], max: vector![x_pos + 12.0, y_pos + 17.0] })
                  .with(animation.down_frames[0].clone()) // Sprite
                  .with(animation)
                  .build();
    }
}
