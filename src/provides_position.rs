// External includes.

// Standard includes.

// Internal includes.
use super::Position;

/// A trait for any type that can provide a [`Position`](struct.Position.html) when queried.
///
/// This trait provides no constraints on the result, save the constraints on the `Position` type itself.
pub trait ProvidesPosition {
    /// Provides a `Position` when called.
    fn provide_position(&self) -> Position;
}

impl<TProvidesPosition> ProvidesPosition for Box<TProvidesPosition>
where
    TProvidesPosition: ProvidesPosition,
{
    fn provide_position(&self) -> Position {
        (**self).provide_position()
    }
}

struct PositionProvider<TFunc>
where
    TFunc: Fn() -> Position,
{
    func: TFunc,
}

impl<TFunc> ProvidesPosition for PositionProvider<TFunc>
where
    TFunc: Fn() -> Position,
{
    fn provide_position(&self) -> Position {
        (self.func)()
    }
}

#[allow(dead_code)]
fn position_provider<TFunc: Fn() -> Position>(t_func: TFunc) -> PositionProvider<TFunc> {
    PositionProvider { func: t_func }
}
