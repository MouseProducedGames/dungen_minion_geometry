// External includes.

// Standard includes.

// Internal includes.
use super::{
    Coord, HasPosition, HasSize, IsArea, IsPosition, IsSize, Length, Position, ProvidesArea, Size,
};

/// Defines an `Area` by a [`Position`](struct.Position.html) and [`Size`](struct.Size.html).
///
/// As such, `Area` has an x and y [`Coord`](type.Coord.html), and a width and height [`Length`](type.Length.html). In the cartesian system used in dungen_minion, and most roguelikes, (x: 0, y: 0) defines the top-left of the coordinate system.
#[derive(Copy, Clone, Debug)]
pub struct Area {
    pos: Position,
    size: Size,
}

impl Area {
    /// Creates a new `Area` with the given [`Position`](struct.Position.html) and [`Size`](struct.Size.html).
    ///
    /// ```
    /// # use dungen_minion_geometry::*;
    /// let value: Area = Area::new(Position::new(5, -3), Size::new(42, 24));
    ///
    /// assert!(*value.pos() == Position::new(5, -3));
    /// assert!(*value.size() == Size::new(42, 24));
    ///
    /// assert!(value.x() == 5);
    /// assert!(value.y() == -3);
    ///
    /// assert!(value.width() == 42);
    /// assert!(value.height() == 24);
    /// ```
    pub fn new(pos: Position, size: Size) -> Self {
        Self { pos, size }
    }
}

impl HasPosition for Area {
    fn pos(&self) -> &Position {
        &self.pos
    }

    fn pos_mut(&mut self) -> &mut Position {
        &mut self.pos
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

impl IsPosition for Area {
    fn x(&self) -> Coord {
        self.pos.x()
    }

    fn x_mut(&mut self) -> &mut Coord {
        self.pos.x_mut()
    }

    fn y(&self) -> Coord {
        self.pos.y()
    }

    fn y_mut(&mut self) -> &mut Coord {
        self.pos.y_mut()
    }
}

impl IsSize for Area {
    fn height(&self) -> Length {
        self.size.height()
    }

    fn height_mut(&mut self) -> &mut Length {
        self.size.height_mut()
    }

    fn width(&self) -> Length {
        self.size.width()
    }

    fn width_mut(&mut self) -> &mut Length {
        self.size.width_mut()
    }
}

impl ProvidesArea for Area {
    fn provide_area(&self) -> Area {
        *self
    }
}
