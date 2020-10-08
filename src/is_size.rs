// External includes.

// Standard includes.

// Internal includes.
use super::Length;

pub trait IsSize {
    fn width(&self) -> Length;

    fn width_mut(&mut self) -> &mut Length;

    fn height(&self) -> Length;

    fn height_mut(&mut self) -> &mut Length;
}
