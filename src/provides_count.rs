// External includes.

// Standard includes.

// Internal includes.
use super::Count;

/// A trait for any type that can provide a [`Size`](struct.Size.html) when queried.
///
/// This trait provides no constraints on the result, save the constraints on the `Size` type itself.
pub trait ProvidesCount {
    /// Provides a `Count` when called.
    fn provide_count(&self) -> Count;
}

impl ProvidesCount for Count {
    fn provide_count(&self) -> Count {
        *self
    }
}

impl<TProvidesCount> ProvidesCount for Box<TProvidesCount>
where
    TProvidesCount: ProvidesCount,
{
    fn provide_count(&self) -> Count {
        (**self).provide_count()
    }
}

struct CountProvider<TFunc>
where
    TFunc: Fn() -> Count,
{
    func: TFunc,
}

impl<TFunc> ProvidesCount for CountProvider<TFunc>
where
    TFunc: Fn() -> Count,
{
    fn provide_count(&self) -> Count {
        (self.func)()
    }
}

#[allow(dead_code)]
fn count_provider<TFunc: Fn() -> Count>(t_func: TFunc) -> CountProvider<TFunc> {
    CountProvider { func: t_func }
}
