use nalgebra::{
    base::Vector2,
    vector,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn unit_vector(&self) -> Vector2<f64> {
        match self {
            Direction::Left => vector![-1.0, 0.0],
            Direction::Right => vector![1.0, 0.0],
            Direction::Up => vector![0.0, -1.0],
            Direction::Down => vector![0.0, 1.0],
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
