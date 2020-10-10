// External includes.

// Standard includes.
use std::ops::{Add, Sub};

// Internal includes.
use super::{Coord, IsPosition, OrdinalRotation};

/// A position on a cartesian coordinate system.
///
/// The x and y components of `Position` can be each be an integer negative, zero, or positive value. In the cartesian system used in dungen_minion, and most roguelikes, (x: 0, y: 0) defines the top-left of the coordinate system.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Position {
    x: Coord,
    y: Coord,
}

impl Position {
    /// Creates a new `Position` from horizontal x and vertical y components.
    ///
    /// ```
    /// # use dungen_minion_geometry::*;
    /// // 5 to the right, 3 up.
    /// let pos: Position = Position::new(5, -3);
    /// assert!(pos.x() == 5);
    /// assert!(pos.y() == -3);
    /// ```
    pub fn new(x: Coord, y: Coord) -> Self {
        Self { x, y }
    }

    /// Returns a copy of `self` after an [`OrdinalRotation`](enum.OrdinalRotation.html).
    ///
    /// ```
    /// # use dungen_minion_geometry::*;
    /// let north_raw: Position = Position::new(0, 1);
    /// let east_raw: Position = Position::new(1, 0);
    /// let south_raw: Position = Position::new(0, -1);
    /// let west_raw: Position = Position::new(-1, 0);
    ///
    /// let north_from_north_raw: Position = north_raw.rotated(OrdinalRotation::None);
    /// let east_from_north_raw: Position = north_raw.rotated(OrdinalRotation::Right90);
    /// let south_from_north_raw: Position = north_raw.rotated(OrdinalRotation::Full180);
    /// let west_from_north_raw: Position = north_raw.rotated(OrdinalRotation::Left90);
    ///
    /// assert!(north_from_north_raw == north_raw);
    /// assert!(east_from_north_raw == east_raw);
    /// assert!(south_from_north_raw == south_raw);
    /// assert!(west_from_north_raw == west_raw);
    /// ```
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
