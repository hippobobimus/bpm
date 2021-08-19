use bevy::math::DVec3;
use std::{
    collections::{HashSet, VecDeque},
};

use crate::direction::Direction;

#[derive(Debug)]
/// Represents either a request to move in a given direction, or to stop.
pub enum MovementCommand {
    Move(DVec3),
    Stop,
}

/// A stack of MovementCommands with the most recent command at its head.
#[derive(Default)]
pub struct MovementCommandStack {
    queue: VecDeque<DVec3>,
    active: HashSet<DVec3>, // used to filter the queue when getting items from stack.
}

impl MovementCommandStack {
    /// Creates a new empty MovementCommandStack.
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

    /// Returns the first Direction in the stack that is still considered active as Some(Direction),
    /// or None if there are no active Directions.
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
