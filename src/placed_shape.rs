// External includes.

// Standard includes.

// Internal includes.
use super::{IntersectsPos, PlacedObject, Shape};

pub trait PlacedShape: IntersectsPos + PlacedObject + Shape {}
