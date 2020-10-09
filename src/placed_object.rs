// External includes.

// Standard includes.

// Internal includes.
use super::{HasArea, Placed};

/// Defines a generic object that has an [`Area`](struct.Area.html) and has been [`Placed`](trait.Placed.html) at a [`Position`](struct.Position.html).
pub trait PlacedObject: HasArea + Placed {}
