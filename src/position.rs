// External includes.

// Standard includes.
use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

// Internal includes.
use super::{
    CardinalDirection, CardinalRotation, Coord, HasPosition, IsPosition, ProvidesPosition,
};

/// A position on a cartesian coordinate system.
///
/// The x and y components of `Position` can be each be an integer negative, zero, or positive value. In the cartesian system used in dungen_minion, and most roguelikes, (x: 0, y: 0) defines the top-left of the coordinate system.
#[derive(AddAssign, Copy, Clone, Debug, Eq, Hash, PartialEq, SubAssign)]
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

    /// Returns a `Position` of (0, 0).
    ///
    /// ```
    /// # use dungen_minion_geometry::Position;
    /// assert!(Position::new(0, 0) == Position::zero());
    /// ```
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    /// Represents a constant with x = 0 and y = -1.
    ///
    /// ```
    /// # use dungen_minion_geometry::Position;
    /// assert!(Position::new(0, -1) == Position::NORTH);
    /// ```
    pub const NORTH: Position = Self { x: 0, y: -1 };

    /// Represents a constant with x = 1 and y = 0.
    ///
    /// ```
    /// # use dungen_minion_geometry::Position;
    /// assert!(Position::new(1, 0) == Position::EAST);
    /// ```
    pub const EAST: Position = Self { x: 1, y: 0 };

    /// Represents a constant with x = 0 and y = 1.
    ///
    /// ```
    /// # use dungen_minion_geometry::Position;
    /// assert!(Position::new(0, 1) == Position::SOUTH);
    /// ```
    pub const SOUTH: Position = Self { x: 0, y: 1 };

    /// Represents a constant with x = -1 and y = 0.
    ///
    /// ```
    /// # use dungen_minion_geometry::Position;
    /// assert!(Position::new(-1, 0) == Position::WEST);
    /// ```
    pub const WEST: Position = Self { x: -1, y: 0 };
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

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "( x: {}, y: {} )", self.x(), self.y())
    }
}

impl From<CardinalDirection> for Position {
    /// Converts a `Position` from an `CardinalDirection`.
    ///
    /// ```
    /// # use dungen_minion_geometry::*;
    /// assert!(Position::from(CardinalDirection::North) == Position::new(0, -1));
    /// assert!(Position::from(CardinalDirection::East) == Position::new(1, 0));
    /// assert!(Position::from(CardinalDirection::South) == Position::new(0, 1));
    /// assert!(Position::from(CardinalDirection::West) == Position::new(-1, 0));
    /// ```
    fn from(ordinal_direction: CardinalDirection) -> Self {
        match ordinal_direction {
            CardinalDirection::North => Position::new(0, -1),
            CardinalDirection::East => Position::new(1, 0),
            CardinalDirection::South => Position::new(0, 1),
            CardinalDirection::West => Position::new(-1, 0),
        }
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

impl Mul<CardinalRotation> for Position {
    type Output = Self;

    /// Returns a copy of `self` after an [`CardinalRotation`](enum.CardinalRotation.html).
    ///
    /// ```
    /// # use dungen_minion_geometry::*;
    /// let north_raw: Position = Position::new(0, 1);
    /// let east_raw: Position = Position::new(1, 0);
    /// let south_raw: Position = Position::new(0, -1);
    /// let west_raw: Position = Position::new(-1, 0);
    ///
    /// let north_from_north_raw: Position = north_raw * CardinalRotation::None;
    /// let east_from_north_raw: Position = north_raw * CardinalRotation::Right90;
    /// let south_from_north_raw: Position = north_raw * CardinalRotation::Full180;
    /// let west_from_north_raw: Position = north_raw * CardinalRotation::Left90;
    ///
    /// assert!(north_from_north_raw == north_raw);
    /// assert!(east_from_north_raw == east_raw);
    /// assert!(south_from_north_raw == south_raw);
    /// assert!(west_from_north_raw == west_raw);
    /// ```
    fn mul(self, rotation: CardinalRotation) -> Self::Output {
        match rotation {
            CardinalRotation::None => self,
            CardinalRotation::Right90 => Self::new(self.y(), -self.x()),
            CardinalRotation::Full180 => Self::new(-self.x(), -self.y()),
            CardinalRotation::Left90 => Self::new(-self.y(), self.x()),
        }
    }
}

impl Neg for Position {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y())
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
