// External includes.

// Standard includes.
use std::fmt;

// Internal includes.
use super::{
    HasShapeArea, HasShapePosition, HasSize, IsShapeArea, IsSize, Length, ProvidesShapeArea,
    ProvidesShapePosition, ProvidesSize, ShapePosition, Size,
};

/// Defines a `ShapeArea` by a [`ShapePosition`](struct.ShapePosition.html) and [`Size`](struct.Size.html).
///
/// As such, `ShapeArea` has an x and y [`Coord`](type.Coord.html) which are each only valid if zero or positive, and a width and height [`Length`](type.Length.html).
///
/// In the cartesian system used in dungen_minion, and most roguelikes, (x: 0, y: 0) defines the top-left of the coordinate system.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct ShapeArea {
    shape_position: ShapePosition,
    size: Size,
}

impl ShapeArea {
    /// Creates a new `ShapeArea` with the given [`ShapePosition`](struct.ShapePosition.html) and [`Size`](struct.Size.html).
    ///
    /// ```
    /// # use dungen_minion_geometry::*;
    /// let value: ShapeArea = ShapeArea::new(ShapePosition::new(5, -3), Size::new(42, 24));
    ///
    /// assert!(*value.shape_position() == ShapePosition::new(5, -3));
    /// assert!(*value.size() == Size::new(42, 24));
    ///
    /// assert!(value.width() == 42);
    /// assert!(value.height() == 24);
    /// ```
    pub fn new(shape_position: ShapePosition, size: Size) -> Self {
        Self {
            shape_position,
            size,
        }
    }
}

impl fmt::Display for ShapeArea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "( ( {} ), ( {} ) )", self.shape_position, self.size)
    }
}

impl From<Size> for ShapeArea {
    /// Creates a new `ShapeArea` from a `ShapePosition` of (0, 0) and the provided `Size`.
    fn from(size: Size) -> Self {
        Self::new(ShapePosition::new(0, 0), size)
    }
}

impl HasShapeArea for ShapeArea {
    fn shape_area(&self) -> &ShapeArea {
        self
    }

    fn shape_area_mut(&mut self) -> &mut ShapeArea {
        self
    }
}

impl HasShapePosition for ShapeArea {
    fn shape_position(&self) -> &ShapePosition {
        &self.shape_position
    }

    fn shape_position_mut(&mut self) -> &mut ShapePosition {
        &mut self.shape_position
    }
}

impl HasSize for ShapeArea {
    fn size(&self) -> &Size {
        &self.size
    }

    fn size_mut(&mut self) -> &mut Size {
        &mut self.size
    }
}

impl IsShapeArea for ShapeArea {}

impl IsSize for ShapeArea {
    fn height(&self) -> Length {
        self.size().height()
    }

    fn height_mut(&mut self) -> &mut Length {
        self.size_mut().height_mut()
    }

    fn width(&self) -> Length {
        self.size().width()
    }

    fn width_mut(&mut self) -> &mut Length {
        self.size_mut().width_mut()
    }
}

impl ProvidesShapeArea for ShapeArea {
    fn provide_shape_area(&self) -> ShapeArea {
        *self
    }
}

impl ProvidesShapePosition for ShapeArea {
    fn provide_shape_position(&self) -> ShapePosition {
        self.shape_position().provide_shape_position()
    }
}

impl ProvidesSize for ShapeArea {
    fn provide_size(&self) -> Size {
        self.size().provide_size()
    }
}
