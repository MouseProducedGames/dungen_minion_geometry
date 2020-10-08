// External includes.

// Standard includes.
use std::ops::{Add, Sub};

// Internal includes.
use super::{HasLocalPosition, Length};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct LocalPosition {
    x: Length,
    y: Length,
}

impl LocalPosition {
    pub fn x(&self) -> Length {
        self.x
    }

    pub fn x_mut(&mut self) -> &mut Length {
        &mut self.x
    }

    pub fn y(&self) -> Length {
        self.y
    }

    pub fn y_mut(&mut self) -> &mut Length {
        &mut self.y
    }

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

impl HasLocalPosition for LocalPosition {
    fn local(&self) -> &Self {
        self
    }

    fn local_mut(&mut self) -> &mut Self {
        self
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
