// External includes.

// Standard includes.
use std::ops::{Add, Sub};

// Internal includes.
use super::{Coord, IsShapePosition};

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

impl Sub for ShapePosition {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
