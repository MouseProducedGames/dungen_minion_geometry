// External includes.

// Standard includes.
use std::ops::{Add, Mul, Sub};

// Internal includes.
use super::{
    Coord, HasPosition, IsPosition, IsShapePosition, OrdinalRotation, ProvidesPosition,
    ShapePosition,
};

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

impl From<ShapePosition> for Position {
    /// Converts a Position directly from a ShapePosition.
    ///
    /// ```
    /// # use dungen_minion_geometry::*;
    /// let shape_position = ShapePosition::new(3, 2);
    /// let position: Position = Position::from(shape_position);
    /// assert!(position.x() == shape_position.x());
    /// assert!(position.y() == shape_position.y());
    /// ```
    fn from(shape_position: ShapePosition) -> Self {
        Position::new(shape_position.x(), shape_position.y())
    }
}

impl HasPosition for Position {
    fn position(&self) -> &Position {
        self
    }

    fn position_mut(&mut self) -> &mut Position {
        self
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

impl Mul<OrdinalRotation> for Position {
    type Output = Self;

    /// Returns a copy of `self` after an [`OrdinalRotation`](enum.OrdinalRotation.html).
    ///
    /// ```
    /// # use dungen_minion_geometry::*;
    /// let north_raw: Position = Position::new(0, 1);
    /// let east_raw: Position = Position::new(1, 0);
    /// let south_raw: Position = Position::new(0, -1);
    /// let west_raw: Position = Position::new(-1, 0);
    ///
    /// let north_from_north_raw: Position = north_raw * OrdinalRotation::None;
    /// let east_from_north_raw: Position = north_raw * OrdinalRotation::Right90;
    /// let south_from_north_raw: Position = north_raw * OrdinalRotation::Full180;
    /// let west_from_north_raw: Position = north_raw * OrdinalRotation::Left90;
    ///
    /// assert!(north_from_north_raw == north_raw);
    /// assert!(east_from_north_raw == east_raw);
    /// assert!(south_from_north_raw == south_raw);
    /// assert!(west_from_north_raw == west_raw);
    /// ```
    fn mul(self, rotation: OrdinalRotation) -> Self::Output {
        match rotation {
            OrdinalRotation::None => self,
            OrdinalRotation::Right90 => Self::new(self.y(), -self.x()),
            OrdinalRotation::Full180 => Self::new(-self.x(), -self.y()),
            OrdinalRotation::Left90 => Self::new(-self.y(), self.x()),
        }
    }
}

impl ProvidesPosition for Position {
    fn provide_position(&self) -> Position {
        *self
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
