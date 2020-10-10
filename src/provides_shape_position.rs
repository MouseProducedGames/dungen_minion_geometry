// External includes.

// Standard includes.

// Internal includes.
use super::ShapePosition;

/// A trait for any type that can provide a [`ShapePosition`](struct.ShapePosition.html) when queried.
///
/// This trait provides no constraints on the result, save the constraints on the `ShapePosition` type itself.
pub trait ProvidesShapePosition {
    /// Provides a `ShapePosition` when called.
    fn provide_shape_position(&self) -> ShapePosition;
}
