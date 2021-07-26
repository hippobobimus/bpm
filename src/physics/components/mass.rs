use crate::constants;

pub struct Mass {
    value: f64,
    inverse: f64,
}

impl Default for Mass {
    fn default() -> Self {
        Self {
            value: constants::DEFAULT_MASS,
            inverse: constants::DEFAULT_INVERSE_MASS,
        }
    }
}

impl Mass {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            inverse: 1.0 / value,
        }
    }

    pub fn from_inverse(inverse: f64) -> Self {
        Self {
            value: 1.0 / inverse,
            inverse,
        }
    }

    pub fn inverse(&self) -> f64 {
        self.inverse
    }

    pub fn is_infinite(&self) -> bool {
        self.inverse == 0.0
    }

    pub fn is_normal(&self) -> bool {
        self.value.is_normal()
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}
