// External includes.

// Standard includes.

// Internal includes.
use super::{ContainsLocalPosition, HasHeight, HasSize, HasWidth, IntersectsLocalPosition};

/// Defines a 2D tile-based shape that does not have a [`Position`](struct.Position.html).
///
/// "shape" is used instead of "polygon" as shapes do not necessarily have regular vertices or edges, nor do they necessarily conform to any regular geometric definition.
pub trait Shape:
    ContainsLocalPosition + HasSize + HasHeight + HasWidth + IntersectsLocalPosition
{
}
