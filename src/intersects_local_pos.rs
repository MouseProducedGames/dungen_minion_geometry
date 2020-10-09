// External includes.

// Standard includes.

// Internal includes.
use super::LocalPosition;

/// Defines an intersection operation between an instance and a [`LocalPosition`](struct.LocalPosition.html).
pub trait IntersectsLocalPos {
    /// Returns true if the given `LocalPosition` is within the boundaries of this instance.
    fn intersects_local_pos(&self, pos: LocalPosition) -> bool;
}
