// External includes.

// Standard includes.
use std::ops::{Add, Sub};

// Internal includes.
use super::{IsLocalPosition, Length};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct LocalPosition {
    x: Length,
    y: Length,
}

impl LocalPosition {
    pub fn new(x: Length, y: Length) -> Self {
        Self { x, y }
    }
}

impl Add for LocalPosition {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl IsLocalPosition for LocalPosition {
    fn x(&self) -> Length {
        self.x
    }

    fn x_mut(&mut self) -> &mut Length {
        &mut self.x
    }

    fn y(&self) -> Length {
        self.y
    }

    fn y_mut(&mut self) -> &mut Length {
        &mut self.y
    }
}

impl Sub for LocalPosition {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
