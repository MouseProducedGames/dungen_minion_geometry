// External includes.

// Standard includes.

// Internal includes.
use super::{IntersectsPos, PlacedObject, Shape};

/// Defines a 2D tile-based shape with a [`Position`](struct.Position.html).
///
/// "shape" is used instead of "polygon" as shapes do not necessarily have regular vertices or edges, nor do they necessarily conform to any regular geometric definition.
pub trait PlacedShape: IntersectsPos + PlacedObject + Shape {
    /// Returns this `PlacedShape` as a `&dyn Shape`.
    fn as_shape(&self) -> &dyn Shape;
}

impl<TShape> PlacedShape for TShape
where
    TShape: IntersectsPos + PlacedObject + Shape,
{
    fn as_shape(&self) -> &dyn Shape {
        self
    }
}
