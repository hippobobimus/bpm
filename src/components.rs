use sdl2::{
    rect::Rect,
};
use specs::prelude::*;
use specs_derive::Component;

use crate::direction::Direction;

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
    pub left_frames: Vec<Sprite>,
    pub right_frames: Vec<Sprite>,
    pub up_frames: Vec<Sprite>,
    pub down_frames: Vec<Sprite>,
}

impl MovementAnimation {
    pub fn new(spritesheet: usize, initial_frame: Rect) -> Self {
        MovementAnimation {
            current_frame: 0,
            left_frames: Self::animation_frames(spritesheet, initial_frame, Direction::Left),
            right_frames: Self::animation_frames(spritesheet, initial_frame, Direction::Right),
            up_frames: Self::animation_frames(spritesheet, initial_frame, Direction::Up),
            down_frames: Self::animation_frames(spritesheet, initial_frame, Direction::Down),
        }
    }

    fn animation_frames(spritesheet: usize, initial_frame: Rect, direction: Direction)
                        -> Vec<Sprite> {
        let (frame_width, frame_height) = initial_frame.size();
    
        let mut frames = Vec::new();

        for i in 0..4 {
            let x_offset = 0; // TODO could be used for different characters
            let y_offset = frame_height as i32 * (Self::spritesheet_row(direction) + (i % 2));

            let region = Rect::new(initial_frame.x() + x_offset,
                                   initial_frame.y() + y_offset,
                                   frame_width,
                                   frame_height);

            let flip_horizontal = Self::flip_frame_horizontal(direction, i);

            frames.push(Sprite { spritesheet, region, flip_horizontal });
        }
    
        frames
    }

    /// Converts a given direction of movement to the row index in the spritesheet containing the
    /// corresponding initial sprite frame.
    fn spritesheet_row(direction: Direction) -> i32 {
        match direction {
            Direction::Left => 2, // Left is the mirror of Right and must be flipped.
            Direction::Right => 2,
            Direction::Up => 4,
            Direction::Down => 0,
        }
    }
    
    /// Left facing sprite is a flipped version of the right facing sprite.
    /// The up/down sprite with one leg raised is flipped for the alternate footstep.
    fn flip_frame_horizontal(direction: Direction, frame: i32) -> bool {
        match direction {
            Direction::Left => true,
            Direction::Up | Direction::Down => {
                frame == 3 // flip on the 3rd frame
            },
            _ => false,
        }
    }
}
