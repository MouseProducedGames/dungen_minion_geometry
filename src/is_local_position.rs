// External includes.

// Standard includes.

// Internal includes.
use super::Length;

pub trait IsLocalPosition {
    fn x(&self) -> Length;

    fn x_mut(&mut self) -> &mut Length;

    fn y(&self) -> Length;

    fn y_mut(&mut self) -> &mut Length;
}
