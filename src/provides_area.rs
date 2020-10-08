// External includes.

// Standard includes.

// Internal includes.
use super::Area;

pub trait ProvidesArea {
    fn provide_area(&self) -> Area;
}
