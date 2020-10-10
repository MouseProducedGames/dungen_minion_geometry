// External includes.

// Standard includes.

// Internal includes.
use super::Size;

/// Defines a type that has a [`Size`](struct.Size.html), but is not necessarily itself a `Size` (see [`IsSize`](trait.IsSize.html)).
pub trait HasSize {
    /// Returns a reference to the instance's `Size`.
    fn size(&self) -> &Size;

    /// Returns a mutable reference to the instance's `Size`.
    fn size_mut(&mut self) -> &mut Size;
}
