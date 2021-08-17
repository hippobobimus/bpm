use crate::constants;

#[derive(Debug)]
/// A component representing the mass of a body.
pub struct Mass {
    value: f64,
    inverse: f64,
}

impl Mass {
    /// Creates a new mass component with the given value.
    pub fn new(value: f64) -> Self {
        Self {
            value,
            inverse: 1.0 / value,
        }
    }

    /// Creates a new mass component with the given inverse mass value.
    pub fn from_inverse(inverse: f64) -> Self {
        Self {
            value: 1.0 / inverse,
            inverse,
        }
    }

    /// Returns the invserse mass value.
    pub fn inverse(&self) -> f64 {
        self.inverse
    }

    /// Returns true if the mass is infinite, otherwise returns false.
    pub fn is_infinite(&self) -> bool {
        self.inverse == 0.0
    }

    /// Returns true if the mass value is 'normal' (neither zero, infinite, subnormal or NaN).
    pub fn is_normal(&self) -> bool {
        self.value.is_normal()
    }

    /// Returns the mass value.
    pub fn value(&self) -> f64 {
        self.value
    }
}

impl Default for Mass {
    fn default() -> Self {
        Self {
            value: constants::DEFAULT_MASS,
            inverse: constants::DEFAULT_INVERSE_MASS,
        }
    }
}
