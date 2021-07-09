use std::{
    collections::{HashSet, VecDeque},
    time::{Duration, Instant},
};

use crate::direction::Direction;

#[derive(Debug)]
pub enum MovementCommand {
    Move(Direction),
    Stop,
}

/// A stack of MovementCommands with the most recent command at its head.
pub struct MovementCommandStack {
    queue: VecDeque<Direction>,
    active: HashSet<Direction>, // used to filter the queue when getting items from stack.
}

impl MovementCommandStack {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            active: HashSet::new(),
        }
    }

    /// Returns the MovementCommand at the head of the stack.
    ///
    /// If the head of the stack has an opposing MovementCommand in the stack, then the Stop
    /// command will be given.
    /// e.g. when the head of the stack is Move(Left) and Move(Right) is also present.
    pub fn get_next(&mut self) -> MovementCommand {
        match self.get_next_active_direction() {
            Some(dir) if !self.active.contains(&dir.get_opposite()) => {
                MovementCommand::Move(dir)
            },
            _ => MovementCommand::Stop,
        }
    }

    fn get_next_active_direction(&mut self) -> Option<Direction> {
        while let Some(dir) = self.queue.front() {
            if self.active.contains(dir) {
                return Some(*dir);
            } else {
                self.queue.pop_front();
            }
        }
        None
    }

    /// Adds a MoveCommand with the given Direction to the head of the stack.
    pub fn add(&mut self, dir: Direction) {
        self.queue.push_front(dir);
        self.active.insert(dir);
    }

    /// Removes any MoveCommands in the given Direction from the stack.
    pub fn remove(&mut self, dir: Direction) {
        self.active.remove(&dir);
    }
}

pub struct DeltaTime {
    dt: Duration,
    instant: Instant,
}

impl DeltaTime {
    pub fn new() -> Self {
        Self {
            dt: Duration::new(0, 0),
            instant: Instant::now(),
        }
    }

    pub fn get_dt(&self) -> &Duration {
        &self.dt
    }

    pub fn update_dt(&mut self) {
        self.dt = self.instant.elapsed();
        self.instant = Instant::now();
    }
}
