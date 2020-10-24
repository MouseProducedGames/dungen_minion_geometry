// External includes.

// Standard includes.

// Internal includes.
use super::Shape;

/// A trait for any type that can provide a [`Shape`](struct.Shape.html) when queried.
///
/// This trait provides no constraints on the result, save the constraints on the `Shape` type itself.
pub trait ProvidesShape {
    /// Provides a `Shape` when called.
    fn provide_shape(&self) -> &dyn Shape;
}

impl<TProvidesShape> ProvidesShape for Box<TProvidesShape>
where
    TProvidesShape: ProvidesShape,
{
    fn provide_shape(&self) -> &dyn Shape {
        (**self).provide_shape()
    }
}

struct ShapeProvider<TFunc>
where
    TFunc: Fn() -> &'static dyn Shape,
{
    func: TFunc,
}

impl<TFunc> ProvidesShape for ShapeProvider<TFunc>
where
    TFunc: Fn() -> &'static dyn Shape,
{
    fn provide_shape(&self) -> &dyn Shape {
        (self.func)()
    }
}

#[allow(dead_code)]
fn shape_provider<TFunc: Fn() -> &'static dyn Shape>(t_func: TFunc) -> ShapeProvider<TFunc> {
    ShapeProvider { func: t_func }
}
