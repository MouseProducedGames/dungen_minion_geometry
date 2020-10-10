// External includes.

// Standard includes.

// Internal includes.
use super::Size;

/// A trait for any type that can provide a [`Size`](struct.Size.html) when queried.
///
/// This trait provides no constraints on the result, save the constraints on the `Size` type itself.
pub trait ProvidesSize {
    /// Provides a `Size` when called.
    fn provide_size(&self) -> Size;
}

impl<TProvidesSize> ProvidesSize for Box<TProvidesSize>
where
    TProvidesSize: ProvidesSize,
{
    fn provide_size(&self) -> Size {
        (**self).provide_size()
    }
}

struct SizeProvider<TFunc>
where
    TFunc: Fn() -> Size,
{
    func: TFunc,
}

impl<TFunc> ProvidesSize for SizeProvider<TFunc>
where
    TFunc: Fn() -> Size,
{
    fn provide_size(&self) -> Size {
        (self.func)()
    }
}

fn size_provider<TFunc: Fn() -> Size>(t_func: TFunc) -> SizeProvider<TFunc> {
    SizeProvider { func: t_func }
}
