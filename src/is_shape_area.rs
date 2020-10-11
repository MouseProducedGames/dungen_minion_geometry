// External includes.

// Standard includes.

// Internal includes.
use super::{Coord, HasShapeArea, IsShapePosition, IsSize, ProvidesShapeArea};

/// `IsArea` is defined as both [`HasShapeArea`](trait.HasShapeArea.html), [`IsSize`](trait.IsSize.html), and [`ProvidesShapeArea`](trait.ProvidesShapeArea.html).
pub trait IsShapeArea: HasShapeArea + IsSize + ProvidesShapeArea {
    /// Obtains the right-most coordinate of the shape area.
    ///
    /// A geometic tile area with a width of 1, has the same right tile as its left tile.
    fn right(&self) -> Coord {
        self.shape_position().x() + (self.width() as Coord - 1).max(0)
    }

    /// Obtains the right-most coordinate of the shape area.
    ///
    /// A geometic tile area with a height of 1, has the same bottom tile as its top tile.
    fn bottom(&self) -> Coord {
        self.shape_position().y() + (self.height() as Coord - 1).max(1)
    }
}
