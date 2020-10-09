// External includes.

// Standard includes.

// Internal includes.
use super::Coord;

/// Designates that the type is definable as a position
///
/// The type has x and y components, in [`Coord`](type.Coord.html) units.  The x and y components of the position can be each be an integer negative, zero, or positive value. No further restrictions are added; for example, something can both be `IsPosition` and have a size ([`IsArea`](trait.IsArea.html) is defined as both `IsPosition` and [`IsSize](trait.IsSize.html)).
pub trait IsPosition {
    /// Returns the horizontal x-component of the position.
    fn x(&self) -> Coord;

    /// Returns a mutable reference to the horizontal x-component of the position.
    fn x_mut(&mut self) -> &mut Coord;

    /// Returns the vertical y-component of the position.
    fn y(&self) -> Coord;

    /// Returns a mutable reference to the vertical y-component of the position.
    fn y_mut(&mut self) -> &mut Coord;
}
