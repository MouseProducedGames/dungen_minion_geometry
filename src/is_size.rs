// External includes.

// Standard includes.

// Internal includes.
use super::{HasHeight, HasSize, HasWidth, ProvidesSize};

/// Designates that the type is definable as a size.
///
/// The type has a given height and width, in [`Length`](type.Length.html) units. The width and height of the size can each be an integer zero, or positive value. No further restrictions are added; for example, something can both be `IsSize` and have a position.
pub trait IsSize: HasHeight + HasSize + HasWidth + ProvidesSize {}
