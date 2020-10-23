// External includes.

// Standard includes.

// Internal includes.
use super::{Coord, HasArea, HasHeight, HasWidth, IsPosition, IsSize, Length, ProvidesArea};

/// `IsArea` is defined as both [`HasPosition`](trait.HasPosition.html), [`HasSize`](trait.HasSize.html), and [`IsSize`](trait.IsSize.html).
pub trait IsArea: HasArea + IsSize + ProvidesArea {
    /// The left-most coordinate of the area.
    ///
    /// Horizontal coordinates increase towards the east.
    fn left(&self) -> Coord {
        self.position().x()
    }

    /// A mutable reference to the left-most coordinate of the area.
    ///
    /// Horizontal coordinates increase towards the east.
    fn left_mut(&mut self) -> &mut Coord {
        self.position_mut().x_mut()
    }

    /// The top-most coordinate of the area.
    ///
    /// Vertical coordinates increase towards the south.
    fn top(&self) -> Coord {
        self.position().y()
    }

    /// A mutable reference to the top-most coordinate of the area.
    ///
    /// Vertical coordinates increase towards the south.
    fn top_mut(&mut self) -> &mut Coord {
        self.position_mut().y_mut()
    }

    /// The right-most coordinate of the area.
    ///
    /// A geometic tile area with a width of 1, has the same right tile as its left tile.
    fn right(&self) -> Coord {
        self.position().x() + (self.width() as Coord - 1).max(0)
    }

    /// Sets the right-most coordinate of the area.
    ///
    /// Cannot set the right-most coordinate to less than the x-coordinate.
    fn right_set(&mut self, value: Coord) {
        let width_coord = (value - self.position().x()) + 1;
        let width = width_coord.max(0) as Length;
        *self.width_mut() = width;
    }

    /// The bottom-most coordinate of the area.
    ///
    /// A geometic tile area with a height of 1, has the same bottom tile as its top tile.
    fn bottom(&self) -> Coord {
        self.position().y() + (self.height() as Coord - 1).max(1)
    }

    /// Sets the bottom-most coordinate of the area.
    ///
    /// Cannot set the bottom-most coordinate to less than the y-coordinate.
    fn bottom_set(&mut self, value: Coord) {
        let height_coord = (value - self.position().y()) + 1;
        let height = height_coord.max(0) as Length;
        *self.height_mut() = height;
    }
}

impl<TIsArea> HasHeight for TIsArea
where
    TIsArea: IsArea,
{
    fn height(&self) -> Length {
        self.size().height()
    }

    fn height_mut(&mut self) -> &mut Length {
        self.size_mut().height_mut()
    }
}

impl<TIsArea> HasWidth for TIsArea
where
    TIsArea: IsArea,
{
    fn width(&self) -> Length {
        self.size().width()
    }

    fn width_mut(&mut self) -> &mut Length {
        self.size_mut().width_mut()
    }
}
