// External includes.

// Standard includes.

// Internal includes.
use super::{HasSize, IntersectsLocalPos};

pub trait Shape: IntersectsLocalPos + HasSize {}
