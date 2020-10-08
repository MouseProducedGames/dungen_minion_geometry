// External includes.

// Standard includes.

// Internal includes.
use super::Position;

pub trait ProvidesPosition {
    fn provide_pos(&self) -> Position;
}
