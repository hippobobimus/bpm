#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

impl Direction {
    pub fn dx(&self) -> i32 {
        match self {
            Direction::LEFT => -1,
            Direction::RIGHT => 1,
            Direction::UP => 0,
            Direction::DOWN => 0,
        }
    }
    pub fn dy(&self) -> i32 {
        match *self {
            Direction::LEFT => 0,
            Direction::RIGHT =>0,
            Direction::UP => -1,
            Direction::DOWN => 1,
        }
    }

    pub fn get_opposite(&self) -> Direction {
        match *self {
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
        }
    }
}
