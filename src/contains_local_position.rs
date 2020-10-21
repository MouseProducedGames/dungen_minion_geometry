// External includes.

// Standard includes.

// Internal includes.
use super::{Containment, Position};

/// Defines a containment operation between an instance and a local [`Position`](struct.Position.html).
pub trait ContainsLocalPosition {
    /// Returns the containment value of the given local `Position` relative to this instance.
    fn contains_local_position(&self, position: Position) -> Containment;
}
