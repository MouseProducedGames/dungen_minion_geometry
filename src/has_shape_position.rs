// External includes.

// Standard includes.

// Internal includes.
use super::ShapePosition;

/// Defines a type that has a [`ShapePosition`](struct.ShapePosition.html), but is not necessarily itself a `ShapePosition` (see [`IsShapePosition`](trait.IsShapePosition.html)).
pub trait HasShapePosition {
    /// Returns a reference to the instance's `ShapePosition`.
    fn shape_position(&self) -> &ShapePosition;

    /// Returns a mutable reference to the instance's `ShapePosition`.
    fn shape_position_mut(&mut self) -> &mut ShapePosition;
}
