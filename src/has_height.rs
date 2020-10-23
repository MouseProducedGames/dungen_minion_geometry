// External includes.

// Standard includes.

// Internal includes.
use super::Length;

/// The trait for objects which have a height.
pub trait HasHeight {
    /// The height of the object.
    fn height(&self) -> Length;

    /// A mutable reference to the height of the object.
    fn height_mut(&mut self) -> &mut Length;
}
