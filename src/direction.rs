#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn dx(&self) -> i32 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
            Direction::Up => 0,
            Direction::Down => 0,
        }
    }
    pub fn dy(&self) -> i32 {
        match *self {
            Direction::Left => 0,
            Direction::Right =>0,
            Direction::Up => -1,
            Direction::Down => 1,
        }
    }

    pub fn get_opposite(&self) -> Direction {
        match *self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}
