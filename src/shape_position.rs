// External includes.

// Standard includes.
use std::convert::From;
use std::fmt;
use std::ops::{Add, Mul, Sub};

// Internal includes.
use super::{
    Coord, HasShapePosition, IsPosition, IsShapePosition, OrdinalDirection, OrdinalRotation,
    Position, ProvidesShapePosition,
};

/// A position relative to the top-left corner of a shape.
///
/// The x and y components of `ShapePosition` can be each be a negative, zero, or positive integer value.
/// In the cartesian system used in dungen_minion, and most roguelikes, (x: 0, y: 0) defines the top-left of the coordinate system; moving down or to the right results in increasingly positive numbers.
///
/// `ShapePosition` is only valid if each of its components are a zero, or positive, value.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct ShapePosition {
    x: Coord,
    y: Coord,
}

impl ShapePosition {
    /// Creates a new `ShapePosition` from horizontal x and vertical y components.
    ///
    /// ```
    /// # use dungen_minion_geometry::*;
    /// // 3 to the right, 2 down.
    /// let shape_position: ShapePosition = ShapePosition::new(3, 2);
    /// assert!(shape_position.x() == 3);
    /// assert!(shape_position.y() == 2);
    /// assert!(shape_position.is_valid_shape_index());
    /// ```
    ///
    /// ```compile_fail
    /// # use dungen_minion_geometry::*;
    /// // You can assign negative numbers for convenience; however, a `ShapePosition` with one or more negative components is not a valid shape position, and cannot be used to index into a shape.
    /// let shape_position_pos_neg: ShapePosition = ShapePosition::new(3, -2);
    /// let shape_position_neg_pos: ShapePosition = ShapePosition::new(-3, 2);
    /// let shape_position_neg_neg: ShapePosition = ShapePosition::new(-3, -2);
    /// // If any of the given shape positions were valid, this assertion would pass.
    /// assert!(
    ///     shape_position_neg_neg.is_valid_shape_index()
    ///     || shape_position_neg_pos.is_valid_shape_index()
    ///     || shape_position_pos_neg().is_valid_shape_index()
    /// );
    /// ```
    pub fn new(x: Coord, y: Coord) -> Self {
        Self { x, y }
    }
}

impl Add for ShapePosition {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl fmt::Display for ShapePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "( x: {}, y: {} ).is_valid_shape_index() -> {}",
            self.x(),
            self.y(),
            self.is_valid_shape_index()
        )
    }
}

impl From<OrdinalDirection> for ShapePosition {
    /// Converts a `ShapePosition` from an `OrdinalDirection`.
    ///
    /// ```
    /// # use dungen_minion_geometry::*;
    /// assert!(ShapePosition::from(OrdinalDirection::North) == ShapePosition::new(0, -1));
    /// assert!(ShapePosition::from(OrdinalDirection::East) == ShapePosition::new(1, 0));
    /// assert!(ShapePosition::from(OrdinalDirection::South) == ShapePosition::new(0, 1));
    /// assert!(ShapePosition::from(OrdinalDirection::West) == ShapePosition::new(-1, 0));
    /// ```
    fn from(ordinal_direction: OrdinalDirection) -> Self {
        match ordinal_direction {
            OrdinalDirection::North => ShapePosition::new(0, -1),
            OrdinalDirection::East => ShapePosition::new(1, 0),
            OrdinalDirection::South => ShapePosition::new(0, 1),
            OrdinalDirection::West => ShapePosition::new(-1, 0),
        }
    }
}

impl From<Position> for ShapePosition {
    /// Converts a `ShapePosition` directly from a `Position`.
    ///
    /// ```
    /// # use dungen_minion_geometry::*;
    /// let position = Position::new(3, 2);
    /// let shape_position: ShapePosition = ShapePosition::from(position);
    /// assert!(shape_position.x() == position.x());
    /// assert!(shape_position.y() == position.y());
    /// ```
    fn from(position: Position) -> Self {
        ShapePosition::new(position.x(), position.y())
    }
}

impl HasShapePosition for ShapePosition {
    fn shape_position(&self) -> &ShapePosition {
        self
    }

    fn shape_position_mut(&mut self) -> &mut ShapePosition {
        self
    }
}

impl IsShapePosition for ShapePosition {
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

impl Mul<OrdinalRotation> for ShapePosition {
    type Output = Self;

    /// Returns a copy of `self` after an [`OrdinalRotation`](enum.OrdinalRotation.html).
    ///
    /// ```
    /// # use dungen_minion_geometry::*;
    /// let north_raw: ShapePosition = ShapePosition::new(0, 1);
    /// let east_raw: ShapePosition = ShapePosition::new(1, 0);
    /// let south_raw: ShapePosition = ShapePosition::new(0, -1);
    /// let west_raw: ShapePosition = ShapePosition::new(-1, 0);
    ///
    /// let north_from_north_raw: ShapePosition = north_raw * OrdinalRotation::None;
    /// let east_from_north_raw: ShapePosition = north_raw * OrdinalRotation::Right90;
    /// let south_from_north_raw: ShapePosition = north_raw * OrdinalRotation::Full180;
    /// let west_from_north_raw: ShapePosition = north_raw * OrdinalRotation::Left90;
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

impl ProvidesShapePosition for ShapePosition {
    fn provide_shape_position(&self) -> ShapePosition {
        *self
    }
}

impl Sub for ShapePosition {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
