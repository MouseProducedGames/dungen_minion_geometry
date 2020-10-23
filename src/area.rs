// External includes.
use rand::{thread_rng, Rng};

// Standard includes.
use std::fmt;

// Internal includes.
use super::{
    Containment, ContainsLocalPosition, HasArea, HasHeight, HasPosition, HasSize, HasWidth,
    IntersectsLocalPosition, IntersectsPosition, IsArea, IsPosition, IsSize, Placed, PlacedObject,
    Position, ProvidesArea, ProvidesPosition, ProvidesSize, Shape, Size,
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

impl ContainsLocalPosition for Area {
    fn contains_local_position(&self, position: Position) -> Containment {
        if position.x() < self.left()
            || position.y() < self.top()
            || position.x() > self.right()
            || position.y() > self.bottom()
        {
            // It's entirely outside.
            Containment::Disjoint
        } else if position.x() > self.left()
            && position.y() > self.top()
            && position.x() < self.right()
            && position.y() < self.bottom()
        {
            // It's entirely inside.
            Containment::Contains
        } else {
            // If it's neither entirely outside nor entirely inside, it intersects.
            Containment::Intersects
        }
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

impl IntersectsLocalPosition for Area {
    fn intersects_local_position(&self, position: Position) -> bool {
        !(position.x() < 0
            || position.y() < 0
            || position.x() as u32 >= self.width()
            || position.y() as u32 >= self.height())
    }
}

impl IntersectsPosition for Area {}

impl IsArea for Area {}

impl IsSize for Area {}

impl Placed for Area {}

impl PlacedObject for Area {}

impl ProvidesArea for Area {
    fn provide_area(&self) -> Area {
        *self
    }
}

impl ProvidesPosition for Area {
    fn provide_position(&self) -> Position {
        self.position
            + Position::new(
                thread_rng().gen_range(0, self.size().width()) as i32,
                thread_rng().gen_range(0, self.size().height()) as i32,
            )
    }
}

impl ProvidesSize for Area {
    fn provide_size(&self) -> Size {
        self.size().provide_size()
    }
}

impl Shape for Area {}
