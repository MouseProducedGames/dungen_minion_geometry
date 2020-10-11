// External includes.

// Standard includes.

// Internal includes.
use super::ShapeArea;

/// A trait for any type that can provide a [`ShapeArea`](struct.ShapeArea.html) when queried.
///
/// This trait provides no constraints on the result, save the constraints on the `ShapeArea` type itself.
pub trait ProvidesShapeArea {
    /// Provides a `ShapeArea` when called.
    fn provide_shape_area(&self) -> ShapeArea;
}

impl<TProvidesShapeArea> ProvidesShapeArea for Box<TProvidesShapeArea>
where
    TProvidesShapeArea: ProvidesShapeArea,
{
    fn provide_shape_area(&self) -> ShapeArea {
        (**self).provide_shape_area()
    }
}

struct ShapeAreaProvider<TFunc>
where
    TFunc: Fn() -> ShapeArea,
{
    func: TFunc,
}

impl<TFunc> ProvidesShapeArea for ShapeAreaProvider<TFunc>
where
    TFunc: Fn() -> ShapeArea,
{
    fn provide_shape_area(&self) -> ShapeArea {
        (self.func)()
    }
}

#[allow(dead_code)]
fn shape_area_provider<TFunc: Fn() -> ShapeArea>(t_func: TFunc) -> ShapeAreaProvider<TFunc> {
    ShapeAreaProvider { func: t_func }
}
