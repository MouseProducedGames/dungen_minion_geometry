// External includes.

// Standard includes.

// Internal includes.
use super::LocalPosition;

/// A trait for any type that can provide a [`LocalPosition`](struct.LocalPosition.html) when queried. 
/// 
/// This trait provides no constraints on the result, save the constraints on the `LocalPosition` type itself.
pub trait ProvidesLocalPosition {
    /// Provides a `LocalPosition` when called.
    fn provide_local(&self) -> LocalPosition;
}
