use crate::direction::Direction;
use sdl2::rect::{Point, Rect};
use std::collections::HashSet;
use std::collections::VecDeque;

const PLAYER_MOVEMENT_SPEED: i32 = 10;

#[derive(Debug)]
pub struct Entity {
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
            direction: Direction::RIGHT,
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
    }

    fn update_velocity(&mut self) {
        while let Some(dir) = self.directions_queue.front() {
            if self.is_direction_active(dir) {
                if self.is_direction_active(&dir.get_opposite()) {
                    break;
                }
                self.direction = *dir;
                self.speed = PLAYER_MOVEMENT_SPEED;
                return;
            } else {
                self.directions_queue.pop_front();
            }
        }
    
        self.speed = 0;
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
}
