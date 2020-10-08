// External includes.

// Standard includes.

// Internal includes.
use super::Coord;

pub trait IsPosition {
    fn x(&self) -> Coord;

    fn x_mut(&mut self) -> &mut Coord;

    fn y(&self) -> Coord;

    fn y_mut(&mut self) -> &mut Coord;
}
