// External includes.

// Standard includes.
use std::fmt;
use std::ops::Mul;

// Internal includes.
use super::{
    Area, CardinalRotation, HasHeight, HasSize, HasWidth, IsSize, Length, ProvidesArea,
    ProvidesSize,
};

/// Defines a `Size` with the given height and width, in [`Length`](type.Length.html) units.
///
/// Size does not have a position, and its width and height can each be an integer zero, or positive value. In the cartesian system used in dungen_minion, and most roguelikes, (x: 0, y: 0) defines the top-left of the coordinate system.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Size {
    height: Length,
    width: Length,
}

impl Size {
    /// Creates a new `Size` with the given height and width.
    ///
    /// ```
    /// # use dungen_minion_geometry::*;
    /// let value: Size = Size::new(42, 24);
    /// assert!(value.width() == 42);
    /// assert!(value.height() == 24);
    /// ```
    pub fn new(width: Length, height: Length) -> Self {
        Self { height, width }
    }

    /// Returns a size of (0, 0).
    pub fn zero() -> Self {
        Self {
            height: 0,
            width: 0,
        }
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "( width: {}, height: {} )", self.width(), self.height())
    }
}

impl HasSize for Size {
    fn size(&self) -> &Size {
        self
    }

    fn size_mut(&mut self) -> &mut Size {
        self
    }
}

impl IsSize for Size {}

impl HasHeight for Size {
    fn height(&self) -> Length {
        self.height
    }

    fn height_mut(&mut self) -> &mut Length {
        &mut self.height
    }
}

impl HasWidth for Size {
    fn width(&self) -> Length {
        self.width
    }

    fn width_mut(&mut self) -> &mut Length {
        &mut self.width
    }
}

impl Mul<CardinalRotation> for Size {
    type Output = Self;

    /// Returns a copy of `self` after an [`CardinalRotation`](enum.CardinalRotation.html).
    ///
    /// ```
    /// # use dungen_minion_geometry::*;
    /// let none_raw: Size = Size::new(8, 6);
    /// let right90_raw: Size = Size::new(6, 8);
    /// let full180_raw: Size = Size::new(8, 6);
    /// let left90_raw: Size = Size::new(6, 8);
    ///
    /// let none_from_none_raw: Size = none_raw * CardinalRotation::None;
    /// let right90_from_none_raw: Size = none_raw * CardinalRotation::Right90;
    /// let full180_from_none_raw: Size = none_raw * CardinalRotation::Full180;
    /// let left90_from_none_raw: Size = none_raw * CardinalRotation::Left90;
    ///
    /// assert!(none_from_none_raw == none_raw);
    /// assert!(right90_from_none_raw == right90_raw);
    /// assert!(full180_from_none_raw == full180_raw);
    /// assert!(left90_from_none_raw == left90_raw);
    /// ```
    fn mul(self, rotation: CardinalRotation) -> Self::Output {
        match rotation {
            CardinalRotation::None => self,
            CardinalRotation::Right90 => Self::new(self.height(), self.width()),
            CardinalRotation::Full180 => self,
            CardinalRotation::Left90 => Self::new(self.height(), self.width()),
        }
    }
}

impl ProvidesArea for Size {
    fn provide_area(&self) -> Area {
        Area::from(*self)
    }
}

impl ProvidesSize for Size {
    fn provide_size(&self) -> Size {
        *self.size()
    }
}
