// External includes.

// Standard includes.
use std::fmt;

// Internal includes.
use super::{
    HasArea, HasPosition, HasSize, IsArea, IsSize, Length, Position, ProvidesArea,
    ProvidesPosition, ProvidesSize, Size,
};

/// Defines an `Area` by a [`Position`](struct.Position.html) and [`Size`](struct.Size.html).
///
/// As such, `Area` has an x and y [`Coord`](type.Coord.html), and a width and height [`Length`](type.Length.html). In the cartesian system used in dungen_minion, and most roguelikes, (x: 0, y: 0) defines the top-left of the coordinate system.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Area {
    position: Position,
    size: Size,
}

impl Area {
    /// Creates a new `Area` with the given [`Position`](struct.Position.html) and [`Size`](struct.Size.html).
    ///
    /// ```
    /// # use dungen_minion_geometry::*;
    /// let value: Area = Area::new(Position::new(5, -3), Size::new(42, 24));
    ///
    /// assert!(*value.position() == Position::new(5, -3));
    /// assert!(*value.size() == Size::new(42, 24));
    ///
    /// assert!(value.width() == 42);
    /// assert!(value.height() == 24);
    /// ```
    pub fn new(position: Position, size: Size) -> Self {
        Self { position, size }
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "( ( {} ), ( {} ) )", self.position, self.size)
    }
}

impl From<Size> for Area {
    fn from(size: Size) -> Area {
        Area::new(Position::zero(), size)
    }
}

impl HasArea for Area {
    fn area(&self) -> &Area {
        self
    }

    fn area_mut(&mut self) -> &mut Area {
        self
    }
}

impl HasPosition for Area {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

impl HasSize for Area {
    fn size(&self) -> &Size {
        &self.size
    }

    fn size_mut(&mut self) -> &mut Size {
        &mut self.size
    }
}

impl IsArea for Area {}

impl IsSize for Area {
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

impl ProvidesArea for Area {
    fn provide_area(&self) -> Area {
        *self
    }
}

impl ProvidesPosition for Area {
    fn provide_position(&self) -> Position {
        self.position().provide_position()
    }
}

impl ProvidesSize for Area {
    fn provide_size(&self) -> Size {
        self.size().provide_size()
    }
}
