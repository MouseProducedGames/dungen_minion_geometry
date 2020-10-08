// External includes.

// Standard includes.

// Internal includes.
use super::{Position, Shape};

pub trait IntersectsLocal<TOtherShape>
where
    TOtherShape: Shape,
{
    fn intersects_local(&self, other_offset: Position, other_shape: &TOtherShape) -> bool;
}
