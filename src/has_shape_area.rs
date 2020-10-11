// External includes.

// Standard includes.

// Internal includes.
use super::{HasShapePosition, HasSize, ShapeArea};

/// Defines a type that has a [`ShapeArea`](struct.ShapeArea.html), but is not necessarily itself a `ShapeArea` (see [`IsShapeArea`](trait.IsShapeArea.html)).
pub trait HasShapeArea: HasShapePosition + HasSize {
    /// Returns a reference to the instance's `ShapeArea`.
    fn shape_area(&self) -> &ShapeArea;

    /// Returns a mutable reference to the instance's `ShapeArea`.
    fn shape_area_mut(&mut self) -> &mut ShapeArea;
}
