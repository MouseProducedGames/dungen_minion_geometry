// External includes.

// Standard includes.
use std::ops::{Add, Sub};

// Internal includes.
use super::{IsLocalPosition, Length};

/// A local position on a cartesian coordinate system restricted to the positive axis.
///
/// The x and y components of `LocalPosition` can be each be an integer zero, or positive value.
/// In the cartesian system used in dungen_minion, and most roguelikes, (x: 0, y: 0) defines the top-left of the coordinate system. 
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct LocalPosition {
    x: Length,
    y: Length,
}

impl LocalPosition {
    /// Creates a new `LocalPosition` from horizontal x and vertical y components.
    ///
    /// ```
    /// # use dungen_minion_geometry::*;
    /// // 5 to the right, 3 down.
    /// let pos: LocalPosition = LocalPosition::new(5, 3);
    /// assert!(pos.x() == 5);
    /// assert!(pos.y() == 3);
    /// ```
    /// 
    /// ```compile_fail
    /// # use dungen_minion_geometry::*;
    /// // You cannot assign negative numbers.
    /// let pos: LocalPosition = LocalPosition::new(-5, -3);
    /// ```
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
