// External includes.

// Standard includes.

// Internal includes.
use super::ShapePosition;

/// A trait for any type that can provide a [`ShapePosition`](struct.ShapePosition.html) when queried.
///
/// This trait provides no constraints on the result, save the constraints on the `ShapePosition` type itself.
pub trait ProvidesShapePosition {
    /// Provides a `ShapePosition` when called.
    fn provide_shape_position(&self) -> ShapePosition;
}

impl<TProvidesShapePosition> ProvidesShapePosition for Box<TProvidesShapePosition>
where
    TProvidesShapePosition: ProvidesShapePosition,
{
    fn provide_shape_position(&self) -> ShapePosition {
        (**self).provide_shape_position()
    }
}

struct ShapePositionProvider<TFunc>
where
    TFunc: Fn() -> ShapePosition,
{
    func: TFunc,
}

impl<TFunc> ProvidesShapePosition for ShapePositionProvider<TFunc>
where
    TFunc: Fn() -> ShapePosition,
{
    fn provide_shape_position(&self) -> ShapePosition {
        (self.func)()
    }
}

#[allow(dead_code)]
fn shape_position_provider<TFunc: Fn() -> ShapePosition>(
    t_func: TFunc,
) -> ShapePositionProvider<TFunc> {
    ShapePositionProvider { func: t_func }
}
