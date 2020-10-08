// External includes.

// Standard includes.

// Internal includes.
use super::Position;

pub trait IntersectsPos {
    fn intersects_pos(&self, pos: Position) -> bool;
}
