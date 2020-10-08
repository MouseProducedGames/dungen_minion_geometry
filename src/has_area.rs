// External includes.

// Standard includes.

// Internal includes.
use super::{Area, HasPosition, HasSize};

pub trait HasArea: HasPosition + HasSize {
    fn area(&self) -> &Area;

    fn area_mut(&mut self) -> &mut Area;
}
