use specs::prelude::*;

use crate::{
    components::*,
    constants,
};

#[derive(SystemData)]
pub struct AnimationData<'a> {
    movement_animation: WriteStorage<'a, MovementAnimation>,
    sprite: WriteStorage<'a, Sprite>,
    propulsion: ReadStorage<'a, Propulsion>,
}

pub struct Animator;

impl<'a> System<'a> for Animator {
    type SystemData = AnimationData<'a>;

    // TODO possible extension: parallel join with rayon
    fn run(&mut self, mut data: Self::SystemData) {
        for (anim, sprite, prop) in (&mut data.movement_animation, &mut data.sprite,
                                     &data.propulsion).join() {
            if prop.x == 0.0 && prop.y == 0.0 {
                continue;
            }

            // only update the animation after specified number of ticks.
            if anim.ticks < constants::TICKS_PER_FRAME {
                anim.ticks += 1;
                continue;
            }
            anim.ticks = 0;

            // get series of frames based on direction of propulsion.
            let frames: &Vec<Sprite> = if prop.x > 0.0 {
                &anim.right_frames
            } else if prop.x < 0.0 {
                &anim.left_frames
            } else if prop.y > 0.0 {
                &anim.down_frames
            } else if prop.y < 0.0 {
                &anim.up_frames
            } else {
                continue;
            };

            // update sprite.
            anim.current_frame = (anim.current_frame + 1) % frames.len();
            *sprite = frames[anim.current_frame].clone();
        }
    }
}
