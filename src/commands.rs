use crate::direction::Direction;

pub enum MovementCommand {
    Move(Direction),
    Stop,
}
