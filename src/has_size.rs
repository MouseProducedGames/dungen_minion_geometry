// External includes.

// Standard includes.

// Internal includes.
use super::Size;

pub trait HasSize {
    fn size(&self) -> &Size;

    fn size_mut(&mut self) -> &mut Size;
}
