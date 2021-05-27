use specs::prelude::*;

use crate::{
    components::*,
    direction::Direction,
};

#[derive(SystemData)]
pub struct AnimationData<'a> {
    movement_animation: WriteStorage<'a, MovementAnimation>,
    sprite: WriteStorage<'a, Sprite>,
    velocity: ReadStorage<'a, Velocity>,
}

pub struct Animator;

impl<'a> System<'a> for Animator {
    type SystemData = AnimationData<'a>;

    // TODO possible extension: parallel join with rayon
    fn run(&mut self, mut data: Self::SystemData) {
        for (anim, sprite, vel) in (&mut data.movement_animation, &mut data.sprite,
                                    &data.velocity).join() {
            if vel.speed == 0 {
                continue;
            }

            let frames = match vel.direction {
                Direction::Left => &anim.left_frames,
                Direction::Right => &anim.right_frames,
                Direction::Up => &anim.up_frames,
                Direction::Down => &anim.down_frames,
            };

            anim.current_frame = (anim.current_frame + 1) % frames.len();
            *sprite = frames[anim.current_frame].clone();
        }
    }
}
