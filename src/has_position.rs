// External includes.

// Standard includes.

// Internal includes.
use super::{Position, ProvidesPosition};

/// Defines a type that has a [`Position`](struct.Position.html), but is not necessarily itself a `Position` (see [`IsPosition`](trait.IsPosition.html)).
pub trait HasPosition: ProvidesPosition {
    /// Returns a reference to the instance's `Position`.
    fn pos(&self) -> &Position;

    /// Returns a mutable reference to the instance's `Position`.
    fn pos_mut(&mut self) -> &mut Position;
}
