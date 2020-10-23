// External includes.

// Standard includes.

// Internal includes.
use super::Length;

/// The trait for objects which have a width.
pub trait HasWidth {
    /// The width of the object.
    fn width(&self) -> Length;

    /// A mutable reference to the width of the object.
    fn width_mut(&mut self) -> &mut Length;
}
