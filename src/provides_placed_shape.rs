// External includes.

// Standard includes.

// Internal includes.
use super::PlacedShape;

/// A trait for any type that can provide a [`PlacedShape`](struct.PlacedShape.html) when queried.
///
/// This trait provides no constraints on the result, save the constraints on the `PlacedShape` type itself.
pub trait ProvidesPlacedShape {
    /// Provides a `PlacedShape` when called.
    fn provide_placed_shape(&self) -> Box<dyn PlacedShape>;
}

impl<TProvidesPlacedShape> ProvidesPlacedShape for Box<TProvidesPlacedShape>
where
    TProvidesPlacedShape: ProvidesPlacedShape,
{
    fn provide_placed_shape(&self) -> Box<dyn PlacedShape> {
        (**self).provide_placed_shape()
    }
}

struct PlacedShapeProvider<TFunc>
where
    TFunc: Fn() -> Box<dyn PlacedShape>,
{
    func: TFunc,
}

impl<TFunc> ProvidesPlacedShape for PlacedShapeProvider<TFunc>
where
    TFunc: Fn() -> Box<dyn PlacedShape>,
{
    fn provide_placed_shape(&self) -> Box<dyn PlacedShape> {
        (self.func)()
    }
}

#[allow(dead_code)]
fn placed_shape_provider<TFunc: Fn() -> Box<dyn PlacedShape>>(
    t_func: TFunc,
) -> PlacedShapeProvider<TFunc> {
    PlacedShapeProvider { func: t_func }
}
