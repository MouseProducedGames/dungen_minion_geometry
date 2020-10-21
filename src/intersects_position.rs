// External includes.

// Standard includes.

// Internal includes.
use super::Position;

/// Defines an intersection operation between an instance and a [`Position`](struct.Position.html).
pub trait IntersectsPosition {
    /// Returns true if the given `Position` is within the boundaries of this instance.
    fn intersects_pos(&self, pos: Position) -> bool;
}
