// External includes.

// Standard includes.

// Internal includes.
use super::PlacedShape;

pub trait Intersects<TOtherPlacedShape>
where
    TOtherPlacedShape: PlacedShape,
{
    fn intersects(&self, other_placed_shape: &TOtherPlacedShape) -> bool;
}
