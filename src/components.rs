use sdl2::{
    rect::{Point, Rect},
};
use specs::prelude::*;
use specs_derive::Component;

use std::collections::HashSet;

use crate::{
    //constants,
    direction::Direction,
};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position(pub Point);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub speed: i32,
    pub direction: Direction,
    pub active_directions: HashSet<Direction>,
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

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ActiveDirections {
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct KeyboardControlled;
