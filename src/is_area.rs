// External includes.

// Standard includes.

// Internal includes.
use super::{HasPosition, HasSize, IsSize};

/// `IsArea` is defined as both [`HasPosition`](trait.HasPosition.html), [`HasSize`](trait.HasSize.html), and [`IsSize`](trait.IsSize.html).
pub trait IsArea: HasPosition + HasSize + IsSize {}
