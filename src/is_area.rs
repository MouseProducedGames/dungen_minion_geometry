// External includes.

// Standard includes.

// Internal includes.
use super::{HasArea, IsSize, ProvidesArea};

/// `IsArea` is defined as both [`HasPosition`](trait.HasPosition.html), [`HasSize`](trait.HasSize.html), and [`IsSize`](trait.IsSize.html).
pub trait IsArea: HasArea + IsSize + ProvidesArea {}
