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
                  .with(KeyboardControlled)
                  .with(Mass { value: mass })
                  .with(Propulsion { x: 0.0, y: 0.0 })
                  .with(Resistance { x: 0.0, y: 0.0 })
                  .with(Position { x: x_pos, y: y_pos })
                  .with(Velocity { x: 0.0, y: 0.0 })
                  .with(player_animation.down_frames[0].clone()) // Sprite
                  .with(player_animation)
                  .build();
    }

    pub fn spawn_npc(&mut self, spritesheet: usize, mass: f64, x_pos: f64, y_pos: f64) {
        let player_animation = MovementAnimation::new(spritesheet,
                                                      *constants::SPRITESHEET_INITIAL_FRAME);
    
        self.world.create_entity()
                  .with(Mass { value: mass })
                  .with(Propulsion { x: 0.0, y: 0.0 })
                  .with(Resistance { x: 0.0, y: 0.0 })
                  .with(Position { x: x_pos, y: y_pos })
                  .with(Velocity { x: 0.0, y: 0.0 })
                  .with(player_animation.down_frames[0].clone()) // Sprite
                  .with(player_animation)
                  .build();
    }
}
