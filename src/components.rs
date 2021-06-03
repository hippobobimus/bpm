use sdl2::{
    rect::Rect,
};
use specs::prelude::*;
use specs_derive::Component;

use crate::{
    constants,
    direction::Direction,
    frames_accumulator::FramesAccumulator,
};

// Marker components

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct KeyboardControlled;

// Standard components

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Mass {
    pub value: f64,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Propulsion {
    pub x: f64,
    pub y: f64,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Resistance {
    pub x: f64,
    pub y: f64,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct Sprite {
    pub spritesheet: usize,
    pub region: Rect,
    pub flip_horizontal: bool,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct MovementAnimation {
    pub current_frame: usize,
    pub frames_accum: FramesAccumulator,
    pub left_frames: Vec<Sprite>,
    pub right_frames: Vec<Sprite>,
    pub up_frames: Vec<Sprite>,
    pub down_frames: Vec<Sprite>,
}

impl MovementAnimation {
    /// Generates movement animation frames from a given spritesheet and initial sprite frame.
    pub fn new(spritesheet: usize, initial_frame: Rect) -> Self {
        MovementAnimation {
            current_frame: 0,
            frames_accum: FramesAccumulator::new(constants::SPRITE_ANIMATION_FPS),
            left_frames: Self::animation_frames(spritesheet, initial_frame, Direction::Left),
            right_frames: Self::animation_frames(spritesheet, initial_frame, Direction::Right),
            up_frames: Self::animation_frames(spritesheet, initial_frame, Direction::Up),
            down_frames: Self::animation_frames(spritesheet, initial_frame, Direction::Down),
        }
    }

    /// Generates a series of animation frames from a spritesheet corresponding to a given
    /// direction of travel.
    fn animation_frames(spritesheet: usize, initial_frame: Rect, direction: Direction)
                        -> Vec<Sprite> {
        let (frame_width, frame_height) = initial_frame.size();
    
        let mut frames = Vec::new();

        // Different columns in spritesheet represent different directions of travel.
        let x_offset = Self::spritesheet_col(direction) * frame_width as i32;

        for i in 0..constants::FRAMES_PER_ANIMATION_CYCLE {
            // advance by one frame in the animation on each loop.
            let y_offset = frame_height as i32 * Self::spritesheet_row(i);

            let region = Rect::new(initial_frame.x() + x_offset,
                                   initial_frame.y() + y_offset,
                                   frame_width,
                                   frame_height);

            let flip_horizontal = false;

            frames.push(Sprite { spritesheet, region, flip_horizontal });
        }
    
        frames
    }

    /// Converts a given direction of movement to the row index in the spritesheet containing the
    /// corresponding initial sprite frame.
    fn spritesheet_col(direction: Direction) -> i32 {
        match direction {
            Direction::Left => 3,
            Direction::Right => 1,
            Direction::Up => 2,
            Direction::Down => 0,
        }
    }

    /// Converts the frame index in a movement animation to the corresponding row in the
    /// spritesheet.
    fn spritesheet_row(frame_index: i32) -> i32 {
        match frame_index {
            0 => 0,
            1 => 1,
            2 => 0, // return to standing before stepping with other foot.
            3 => 2,
            _ => 0,
        }
    }
}
