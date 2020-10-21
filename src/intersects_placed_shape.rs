// External includes.

// Standard includes.

// Internal includes.
use super::PlacedShape;

/// Defines an intersection operation between this instance and a [`PlacedShape`](trait.PlacedShape.html).
pub trait IntersectsPlacedShape {
    /// Returns true if the given 'PlacedShape' at is within the boundaries of this instance.
    fn intersects_placed_shape(&self, placed_shape: &dyn PlacedShape) -> bool;
}
