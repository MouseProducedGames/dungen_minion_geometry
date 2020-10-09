// External includes.

// Standard includes.

// Internal includes.
use super::Position;

/// A trait for any type that can provide a [`Position`](struct.Position.html) when queried. 
/// 
/// This trait provides no constraints on the result, save the constraints on the `Position` type itself.
pub trait ProvidesPosition {
    /// Provides a `Position` when called.
    fn provide_pos(&self) -> Position;
}
