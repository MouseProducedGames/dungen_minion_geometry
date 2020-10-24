// External includes.

// Standard includes.

// Internal includes.
use super::{Coord, HasPosition, HasWidth, IsPosition};

/// The trait for objects which have a left coordinate.
///
/// For objects which implement [`HasPosition`](struct.HasPosition.html) and [`HasWidth`](trait.HasWidth.html), the left coordinate is equal to `(self.position().x()`. In this case, `HasWidth` is a bound ensuring that the object can sensibly have a `self.left()`.
pub trait HasLeft {
    /// The left-most coordinate of the object.
    ///
    /// Horizontal coordinates increase towards the east.
    fn left(&self) -> Coord;

    /// A mutable reference to the left-most coordinate of the object.
    ///
    /// Horizontal coordinates increase towards the east.
    fn left_mut(&mut self) -> &mut Coord;
}

impl<THasPositionAndHasWidth> HasLeft for THasPositionAndHasWidth
where
    THasPositionAndHasWidth: HasPosition + HasWidth,
{
    fn left(&self) -> Coord {
        self.position().x()
    }

    fn left_mut(&mut self) -> &mut Coord {
        self.position_mut().x_mut()
    }
}
