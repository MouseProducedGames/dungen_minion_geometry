// External includes.

// Standard includes.

// Internal includes.
use super::{
    ContainsPosition, HasBottom, HasLeft, HasRight, HasTop, IntersectsPosition, PlacedObject, Shape,
};

/// Defines a 2D tile-based shape with a [`Position`](struct.Position.html).
///
/// "shape" is used instead of "polygon" as shapes do not necessarily have regular vertices or edges, nor do they necessarily conform to any regular geometric definition.
pub trait PlacedShape:
    ContainsPosition
    + HasBottom
    + HasLeft
    + HasRight
    + HasTop
    + IntersectsPosition
    + PlacedObject
    + Shape
{
    /// A helper method to allow structs implementing `Map` to be `Clone`'ed.
    ///
    /// [https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/5](https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/5)
    fn box_placed_shape_clone(&self) -> Box<dyn PlacedShape>;

    /// Returns this `PlacedShape` as a `&dyn Shape`.
    fn as_shape(&self) -> &dyn Shape;
}

impl Clone for Box<dyn PlacedShape> {
    fn clone(&self) -> Box<dyn PlacedShape> {
        self.box_placed_shape_clone()
    }
}

impl<TShape: 'static> PlacedShape for TShape
where
    TShape: Clone + ContainsPosition + IntersectsPosition + PlacedObject + Shape,
{
    fn box_placed_shape_clone(&self) -> Box<dyn PlacedShape> {
        Box::new((*self).clone())
    }

    fn as_shape(&self) -> &dyn Shape {
        self
    }
}
