// External includes.

// Standard includes.

// Internal includes.
use super::{Coord, HasShapePosition, ProvidesShapePosition};

/// Designates that the type is definable as a position on a shape.
///
/// The type has x and y components, in [`Coord`](type.Coord.html) units.  Each component of the shape position may each be a negative, zero, or positive, integer value. However, a shape position is only valid if each of its components is either a zero, or positive, integer value.
///
/// No further restrictions are added; for example, something can both be `IsShapePosition` and have a size.
pub trait IsShapePosition: HasShapePosition + ProvidesShapePosition {
    /// Checks that the shape position represents a valid index into a shape. This test will only pass if both the self.x() and self.y() coordinates are either zero, or a positive, integer value.
    fn is_valid_shape_index(&self) -> bool {
        self.x() >= 0 && self.y() >= 0
    }

    /// Returns the horizontal, non-negative, x-component of the shape position.
    fn x(&self) -> Coord;

    /// Returns a mutable reference to the horizontal, non-negative, x-component of the shape position.
    fn x_mut(&mut self) -> &mut Coord;

    /// Returns the horizontal, non-negative, y-component of the shape position.
    fn y(&self) -> Coord;

    /// Returns a mutable reference to the horizontal, non-negative, y-component of the shape position.
    fn y_mut(&mut self) -> &mut Coord;
}
