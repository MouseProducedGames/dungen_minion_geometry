// External includes.

// Standard includes.

// Internal includes.
use super::Size;

pub trait ProvidesSize {
    fn provide_size(&self) -> Size;
}
