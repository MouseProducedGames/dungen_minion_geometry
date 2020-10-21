// External includes.

// Standard includes.

// Internal includes.
use super::{Position, Shape};

/// Defines an intersection operation between this instance and a [`Shape`](trait.Shape.html) at a specific [`Position`](struct.Position.html).
pub trait IntersectsShape<TOtherShape>
where
    TOtherShape: Shape,
{
    /// Returns true if the given 'Shape' at the given `Position` offset is within the boundaries of this instance.
    fn intersects_local(&self, other_offset: Position, other_shape: &TOtherShape) -> bool;
}
