use sdl2::rect::{Point, Rect};
use std::collections::{
    HashSet,
    VecDeque,
};
use crate::{
    constants,
    direction::Direction,
};

const PLAYER_MOVEMENT_SPEED: i32 = 10;

#[derive(Debug)]
pub struct Entity {
    current_frame: i32,
    direction: Direction,
    directions_queue: VecDeque<Direction>,
    directions_active: HashSet<Direction>,
    pub position: Point,
    speed: i32,
    pub sprite: Rect,
}

impl Entity {
    pub fn new(position: Point, sprite: Rect) -> Entity {
        Entity {
            current_frame: 0,
            direction: constants::DEFAULT_ENTITY_DIRECTION,
            directions_queue: VecDeque::new(),
            directions_active: HashSet::new(),
            position: position,
            speed: 0,
            sprite: sprite,
        }
    }

    pub fn update(&mut self) {
        self.update_velocity();
        self.update_position();
        self.current_frame = (self.current_frame + 1) % 4;
    }

    fn update_velocity(&mut self) {
        self.update_direction();
        self.speed = if self.is_direction_active(&self.direction) &&
            !self.is_direction_active(&self.direction.get_opposite()) {
            PLAYER_MOVEMENT_SPEED
        } else {
            0
        }
    }

    /// Changes the current direction of the entity to the next active direction in the queue.
    /// If there are no active directions in the queue then the direction is unchanged.
    fn update_direction(&mut self) {
        while let Some(dir) = self.directions_queue.front() {
            if self.is_direction_active(dir) {
                self.direction = *dir;
                return;
            } else {
                self.directions_queue.pop_front();
            }
        }
    }

    fn update_position(&mut self) {
        self.position = self.position.offset(self.speed * self.direction.dx(), self.speed * self.direction.dy());
    }

    pub fn activate_direction(&mut self, direction: Direction) {
        self.directions_queue.push_front(direction);
        self.directions_active.insert(direction);
    }

    pub fn deactivate_direction(&mut self, direction: Direction) {
        self.directions_active.remove(&direction);
    }

    fn is_direction_active(&self, direction: &Direction) -> bool {
        self.directions_active.contains(direction)
    }

    fn direction_spritesheet_row(&self) -> i32 {
        match self.direction {
            Direction::Left => 2,
            Direction::Right => 2,
            Direction::Up => 4,
            Direction::Down => 0,
        }
    }

    /// Left facing sprite is a flipped version of the right facing sprite.
    /// The up/down sprite with one leg raised is flipped for the alternate footstep.
    pub fn flip_frame_horizontal(&self) -> bool {
        match self.direction {
            Direction::Left => true,
            Direction::Up  => {
                if self.current_frame == 3 as i32 {
                    true
                } else {
                    false
                }
            },
            Direction::Down => {
                if self.current_frame == 3 as i32 {
                    true
                } else {
                    false
                }
            },
            _ => false,
        }
    }

    pub fn current_frame(&self) -> Rect {
        let index = self.direction_spritesheet_row() + (self.current_frame % 2);

        let (frame_width, frame_height) = self.sprite.size();
        Rect::new(
            self.sprite.x(), // + frame_width as i32 * self.current_frame,
            self.sprite.y() + frame_height as i32 * index,
            frame_width,
            frame_height,
        )
    }
}
