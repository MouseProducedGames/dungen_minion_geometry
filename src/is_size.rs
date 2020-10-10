// External includes.

// Standard includes.

// Internal includes.
use super::{HasSize, Length};

/// Designates that the type is definable as a size.
///
/// The type has a given height and width, in [`Length`](type.Length.html) units. The width and height of the size can each be an integer zero, or positive value. No further restrictions are added; for example, something can both be `IsSize` and have a position.
pub trait IsSize: HasSize {
    /// Returns the width of the size.
    fn width(&self) -> Length;

    /// Returns a mutable reference to the width of the size.
    fn width_mut(&mut self) -> &mut Length;

    /// Returns the height of the size.
    fn height(&self) -> Length;

    /// Returns a mutable reference to the height of the size.
    fn height_mut(&mut self) -> &mut Length;
}
