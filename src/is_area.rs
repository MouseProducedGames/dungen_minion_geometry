// External includes.

// Standard includes.

// Internal includes.
use super::{Coord, HasArea, IsPosition, IsSize, ProvidesArea};

/// `IsArea` is defined as both [`HasPosition`](trait.HasPosition.html), [`HasSize`](trait.HasSize.html), and [`IsSize`](trait.IsSize.html).
pub trait IsArea: HasArea + IsSize + ProvidesArea {
    /// Obtains the right-most coordinate of the area.
    ///
    /// A geometic tile area with a width of 1, has the same right tile as its left tile.
    fn right(&self) -> Coord {
        self.position().x() + (self.width() as Coord - 1).max(0)
    }

    /// Obtains the right-most coordinate of the area.
    ///
    /// A geometic tile area with a height of 1, has the same bottom tile as its top tile.
    fn bottom(&self) -> Coord {
        self.position().y() + (self.height() as Coord - 1).max(1)
    }
}
