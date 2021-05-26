use std::collections::{
    HashSet,
    VecDeque,
};

use crate::direction::Direction;

pub enum MovementCommand {
    Move(Direction),
    Stop,
}

pub struct MovementCommandQueue {
    queue: VecDeque<Direction>,
    active: HashSet<Direction>,
}

impl MovementCommandQueue {
    pub fn new() -> MovementCommandQueue {
        MovementCommandQueue {
            queue: VecDeque::new(),
            active: HashSet::new(),
        }
    }

    pub fn get_next(&mut self) -> MovementCommand {
        while let Some(dir) = self.queue.front() {
            if self.active.contains(dir) {
                return MovementCommand::Move(*dir);
            } else {
                self.queue.pop_front();
            }
        }
        MovementCommand::Stop
    }

    pub fn add(&mut self, dir: Direction) {
        self.queue.push_front(dir);
        self.active.insert(dir);
    }

    pub fn remove(&mut self, dir: Direction) {
        self.active.remove(&dir);
    }
}
