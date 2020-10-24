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
    /// A helper method to allow structs implementing `Map` to be `Clone`'ed.
    ///
    /// [https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/5](https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/5)
    fn box_shape_clone(&self) -> Box<dyn Shape>;
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Box<dyn Shape> {
        self.box_shape_clone()
    }
}
