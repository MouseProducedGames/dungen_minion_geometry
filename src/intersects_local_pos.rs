// External includes.

// Standard includes.

// Internal includes.
use super::LocalPosition;

pub trait IntersectsLocalPos {
    fn intersects_local_pos(&self, pos: LocalPosition) -> bool;
}
