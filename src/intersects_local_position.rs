// External includes.

// Standard includes.

// Internal includes.
use super::Position;

/// Defines an intersection operation between an instance and a local [`Position`](struct.Position.html).
pub trait IntersectsLocalPosition {
    /// Returns true if the given local `Position` is within the boundaries of this instance.
    fn intersects_local_position(&self, position: Position) -> bool;
}
