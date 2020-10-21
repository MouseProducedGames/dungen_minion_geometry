// External includes.

// Standard includes.

// Internal includes.
use super::{Containment, ContainsLocalPosition, HasPosition, Position};

/// Defines a containment operation between an instance and a [`Position`](struct.Position.html).
pub trait ContainsPosition: HasPosition + ContainsLocalPosition {
    /// Returns the containment value of the given `Position` relative to this instance.
    fn contains_position(&self, position: Position) -> Containment {
        self.contains_local_position(position - *self.position())
    }
}
