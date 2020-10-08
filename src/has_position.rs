// External includes.

// Standard includes.

// Internal includes.
use super::Position;

pub trait HasPosition {
    fn pos(&self) -> &Position;

    fn pos_mut(&mut self) -> &mut Position;
}
