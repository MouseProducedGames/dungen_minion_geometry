// External includes.

// Standard includes.

// Internal includes.
use super::Length;

/// Designates that the type is definable as a local position.
///
/// The type has x and y components, in [`Length`](type.Length.html) units.  The x and y components of the local position can be each be an integer zero, or positive value. No further restrictions are added; for example, something can both be `IsLocalPosition` and have a size.
pub trait IsLocalPosition {
    /// Returns the horizontal, non-negative, x-component of the local position.
    fn x(&self) -> Length;

    /// Returns a mutable reference to the horizontal, non-negative, x-component of the local position.
    fn x_mut(&mut self) -> &mut Length;

    /// Returns the horizontal, non-negative, y-component of the local position.
    fn y(&self) -> Length;

    /// Returns a mutable reference to the horizontal, non-negative, y-component of the local position.
    fn y_mut(&mut self) -> &mut Length;
}
