// External includes.

// Standard includes.
use std::cmp::{Ordering, PartialOrd};

// Internal includes.

/// An enum that specifies the intersection of two shapes to a more precise level of detail.
#[derive(Copy, Clone, Debug, Display, Eq, PartialEq)]
pub enum Containment {
    /// The shapes are not intersecting.
    Disjoint,
    /// The intersecting shape is only partially contained within this shape.
    Intersects,
    /// The intersecting shape is entirely contained within this shape.
    Contains,
}

impl From<i8> for Containment {
    fn from(value: i8) -> Self {
        match value {
            0 => Containment::Disjoint,
            1 => Containment::Intersects,
            2 => Containment::Contains,
            _ => panic!("Cannot convert {} to Containment; range is [0..2].", value),
        }
    }
}

impl From<Containment> for i8 {
    fn from(value: Containment) -> Self {
        match value {
            Containment::Disjoint => 0,
            Containment::Intersects => 1,
            Containment::Contains => 2,
        }
    }
}

impl Ord for Containment {
    fn cmp(&self, other: &Self) -> Ordering {
        i8::from(*self).cmp(&i8::from(*other))
    }
}

impl PartialOrd for Containment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        i8::from(*self).partial_cmp(&i8::from(*other))
    }
}
