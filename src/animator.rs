use specs::prelude::*;

use crate::{
    components::*,
    resources::*,
};

#[derive(SystemData)]
pub struct AnimationData<'a> {
    movement_animation: WriteStorage<'a, MovementAnimation>,
    sprite: WriteStorage<'a, Sprite>,
    propulsion: ReadStorage<'a, Propulsion>,
    delta_time: ReadExpect<'a, DeltaTime>,
}

pub struct Animator;

impl<'a> System<'a> for Animator {
    type SystemData = AnimationData<'a>;

    // TODO possible extension: parallel join with rayon
    fn run(&mut self, mut data: Self::SystemData) {
        let dt = *data.delta_time.get_dt();

        for (anim, sprite, prop) in (&mut data.movement_animation, &mut data.sprite,
                                     &data.propulsion).join() {
            // do not progress animation when propulsion force is 0.
            if prop.x == 0.0 && prop.y == 0.0 {
                continue;
            }

            anim.frames_accum.increment(dt);

            let whole_frames = anim.frames_accum.get_whole_frames();

            // only advance animation once sufficient time has passed for the next frame.
            if whole_frames > 0 {
                // get series of animation frames based on direction of propulsion.
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

                // update sprite by number of elapsed whole animation frames since last update.
                anim.current_frame = (anim.current_frame + whole_frames as usize) % frames.len();
                *sprite = frames[anim.current_frame].clone();

                // reset whole frame part of frame accumulator.
                anim.frames_accum.reset_whole_frames();
            }
        }
    }
}
