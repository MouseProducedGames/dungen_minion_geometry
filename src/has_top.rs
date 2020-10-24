// External includes.

// Standard includes.

// Internal includes.
use super::{Coord, HasHeight, HasPosition, IsPosition};

/// The trait for objects which have a top coordinate.
///
/// For objects which implement [`HasPosition`](trait.HasPosition.html) and [`HasHeight`](trait.HasHeight.html), the top coordinate is equal to `(self.position().y()`. In this case, `HasHeight` is a bound ensuring that the object can sensibly have a `self.top()`.
pub trait HasTop {
    /// The top-most coordinate of the area.
    ///
    /// Vertical coordinates increase towards the south.
    fn top(&self) -> Coord;

    /// A mutable reference to the top-most coordinate of the area.
    ///
    /// Vertical coordinates increase towards the south.
    fn top_mut(&mut self) -> &mut Coord;
}

impl<THasPositionAndHasHeight> HasTop for THasPositionAndHasHeight
where
    THasPositionAndHasHeight: HasPosition + HasHeight,
{
    fn top(&self) -> Coord {
        self.position().y()
    }

    fn top_mut(&mut self) -> &mut Coord {
        self.position_mut().y_mut()
    }
}
