// External includes.

// Standard includes.

// Internal includes.
use super::ShapePosition;

/// Defines an intersection operation between an instance and a [`ShapePosition`](struct.ShapePosition.html).
pub trait IntersectsShapePosition {
    /// Returns true if the given `ShapePosition` is within the boundaries of this instance.
    fn intersects_shape_position(&self, pos: ShapePosition) -> bool;
}
