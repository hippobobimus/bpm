use specs::prelude::*;

use crate::{
    components::*,
    direction::Direction,
};

pub struct Animator;

impl<'a> System<'a> for Animator {
    type SystemData = (
        WriteStorage<'a, MovementAnimation>,
        WriteStorage<'a, Sprite>,
        ReadStorage<'a, Velocity>,
    );

    // possible extension: parallel join with rayon
    fn run(&mut self, mut data: Self::SystemData) {
        for (anim, sprite, vel) in (&mut data.0, &mut data.1, &data.2).join() {
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
