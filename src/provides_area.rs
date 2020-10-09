// External includes.

// Standard includes.

// Internal includes.
use super::Area;

/// A trait for any type that can provide a [`Area`](struct.Area.html) when queried. 
/// 
/// This trait provides no constraints on the result, save the constraints on the `Area` type itself.
pub trait ProvidesArea {
    /// Provides a `Area` when called.
    fn provide_area(&self) -> Area;
}
