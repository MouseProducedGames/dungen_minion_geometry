// External includes.

// Standard includes.

// Internal includes.
use super::{HasPosition, IntersectsLocalPosition, Position};

/// Defines an intersection operation between an instance and a [`Position`](struct.Position.html).
pub trait IntersectsPosition: HasPosition + IntersectsLocalPosition {
    /// Returns true if the given `Position` is within the boundaries of this instance.
    fn intersects_position(&self, position: Position) -> bool {
        self.intersects_local_position(position - *self.position())
    }
}
