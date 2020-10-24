// External includes.

// Standard includes.

// Internal includes.
use super::{Coord, HasHeight, HasPosition, IsPosition, Length};

/// The trait for objects which have a bottom coordinate.
///
/// For objects which implement [`HasPosition`](struct.HasPosition.html) and [`HasHeight`](trait.HasHeight.html), the bottom coordinate is equal to `self.position().y() + (self.height() as Coord - 1).max(0)`.
pub trait HasBottom {
    /// The bottom-most coordinate of the object.
    ///
    /// A geometic tile area with a height of 1, has the same bottom tile as its top tile.
    fn bottom(&self) -> Coord;

    /// Sets the bottom-most coordinate of the object.
    ///
    /// Cannot set the bottom-most coordinate to less than the top coordinate.
    fn bottom_set(&mut self, value: Coord);
}

impl<THasPositionAndHasHeight> HasBottom for THasPositionAndHasHeight
where
    THasPositionAndHasHeight: HasPosition + HasHeight,
{
    fn bottom(&self) -> Coord {
        self.position().y() + (self.height() as Coord - 1).max(0)
    }

    fn bottom_set(&mut self, value: Coord) {
        let height_coord = (value - self.position().y()) + 1;
        let height = height_coord.max(0) as Length;
        *self.height_mut() = height;
    }
}
