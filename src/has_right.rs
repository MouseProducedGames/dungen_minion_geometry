// External includes.

// Standard includes.

// Internal includes.
use super::{Coord, HasPosition, HasWidth, IsPosition, Length};

/// The trait for objects which have a right coordinate.
///
/// For objects which implement [`HasPosition`](struct.HasPosition.html) and [`HasWidth`](trait.HasWidth.html), the right coordinate is equal to `self.position().x() + (self.width() as Coord - 1).max(0)`.
pub trait HasRight {
    /// The right-most coordinate of the object.
    ///
    /// A geometic tile area with a width of 1, has the same right tile as its left tile.
    fn right(&self) -> Coord;

    /// Sets the right-most coordinate of the object.
    ///
    /// Cannot set the right-most coordinate to less than the x-coordinate.
    fn right_set(&mut self, value: Coord);
}

impl<THasPositionAndHasWidth> HasRight for THasPositionAndHasWidth
where
    THasPositionAndHasWidth: HasPosition + HasWidth,
{
    fn right(&self) -> Coord {
        self.position().x() + (self.width() as Coord - 1).max(0)
    }

    fn right_set(&mut self, value: Coord) {
        let width_coord = (value - self.position().x()) + 1;
        let width = width_coord.max(0) as Length;
        *self.width_mut() = width;
    }
}
