// External includes.

// Standard includes.

// Internal includes.
use super::PlacedShape;

/// Defines an intersection operation between this instance and a [`PlacedShape`](trait.PlacedShape.html).
pub trait IntersectsPlacedShape<TOtherPlacedShape>
where
    TOtherPlacedShape: PlacedShape,
{
    /// Returns true if the given 'PlacedShape' at the given offset is within the boundaries of this instance.
    fn intersects(&self, other_placed_shape: &TOtherPlacedShape) -> bool;
}
