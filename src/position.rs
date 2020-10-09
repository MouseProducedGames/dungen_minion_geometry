// External includes.

// Standard includes.
use std::ops::{Add, Sub};

// Internal includes.
use super::{Coord, IsPosition, OrdinalRotation};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Position {
    x: Coord,
    y: Coord,
}

impl Position {
    pub fn new(x: Coord, y: Coord) -> Self {
        Self { x, y }
    }

    pub fn rotated(&self, rotation: OrdinalRotation) -> Self {
        match rotation {
            OrdinalRotation::None => *self,
            OrdinalRotation::Right90 => Self::new(self.y(), -self.x()),
            OrdinalRotation::Full180 => Self::new(-self.x(), -self.y()),
            OrdinalRotation::Left90 => Self::new(-self.y(), self.x()),
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl IsPosition for Position {
    fn x(&self) -> Coord {
        self.x
    }

    fn x_mut(&mut self) -> &mut Coord {
        &mut self.x
    }

    fn y(&self) -> Coord {
        self.y
    }

    fn y_mut(&mut self) -> &mut Coord {
        &mut self.y
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
