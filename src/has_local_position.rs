// External includes.

// Standard includes.

// Internal includes.
use super::LocalPosition;

/// Defines a type that has a [`LocalPosition`](struct.LocalPosition.html), but is not necessarily itself a `LocalPosition` (see [`IsLocalPosition`](trait.IsLocalPosition.html)).
pub trait HasLocalPosition {
    /// Returns a reference to the instance's `LocalPosition`.
    fn local(&self) -> &LocalPosition;

    /// Returns a mutable reference to the instance's `LocalPosition`.
    fn local_mut(&mut self) -> &mut LocalPosition;
}
