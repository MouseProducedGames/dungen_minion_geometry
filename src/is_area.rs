// External includes.

// Standard includes.

// Internal includes.
use super::{IsPosition, IsSize};

/// `IsArea` is defined as both [`IsPosition`](trait.IsPosition.html) and [`IsSize`](trait.IsSize.html)
pub trait IsArea: IsPosition + IsSize {}
