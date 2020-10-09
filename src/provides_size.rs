// External includes.

// Standard includes.

// Internal includes.
use super::Size;

/// A trait for any type that can provide a [`Size`](struct.Size.html) when queried.
/// 
/// This trait provides no constraints on the result, save the constraints on the `Size` type itself.
pub trait ProvidesSize {
    /// Provides a `Size` when called.
    fn provide_size(&self) -> Size;
}
