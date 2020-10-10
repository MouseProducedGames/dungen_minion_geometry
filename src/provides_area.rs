// External includes.

// Standard includes.

// Internal includes.
use super::Area;

/// A trait for any type that can provide a [`Area`](struct.Area.html) when queried.
///
/// This trait provides no constraints on the result, save the constraints on the `Area` type itself.
pub trait ProvidesArea {
    /// Provides a `Area` when called.
    fn provide_area(&self) -> Area;
}

impl<TProvidesArea> ProvidesArea for Box<TProvidesArea>
where
    TProvidesArea: ProvidesArea,
{
    fn provide_area(&self) -> Area {
        (**self).provide_area()
    }
}

struct AreaProvider<TFunc>
where
    TFunc: Fn() -> Area,
{
    func: TFunc,
}

impl<TFunc> ProvidesArea for AreaProvider<TFunc>
where
    TFunc: Fn() -> Area,
{
    fn provide_area(&self) -> Area {
        (self.func)()
    }
}

#[allow(dead_code)]
fn area_provider<TFunc: Fn() -> Area>(t_func: TFunc) -> AreaProvider<TFunc> {
    AreaProvider { func: t_func }
}
