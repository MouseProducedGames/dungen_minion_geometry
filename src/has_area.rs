// External includes.

// Standard includes.

// Internal includes.
use super::{Area, HasPosition, HasSize};

/// Defines a type that has an [`Area`](struct.Area.html), but is not necessarily itself an `Area` (see [`IsArea`](trait.IsArea.html)).
pub trait HasArea: HasPosition + HasSize {
    /// Returns a reference to the instance's `Area`.
    fn area(&self) -> &Area;

    /// Returns a mutable reference to the instance's `Area`.
    fn area_mut(&mut self) -> &mut Area;
}
