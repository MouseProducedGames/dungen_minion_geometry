// External includes.

// Standard includes.

// Internal includes.
use super::{HasShapeArea, IsSize, ProvidesShapeArea};

/// `IsArea` is defined as both [`HasShapeArea`](trait.HasShapeArea.html), [`IsSize`](trait.IsSize.html), and [`ProvidesShapeArea`](trait.ProvidesShapeArea.html).
pub trait IsShapeArea: HasShapeArea + IsSize + ProvidesShapeArea {}
