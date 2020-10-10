// External includes.

// Standard includes.
use std::convert::From;
use std::ops::{Add, Sub};

// Internal includes.
use super::OrdinalRotation;

/// Defines a direction on a cartesian plane where each direction is an orthogonal 90-degree vector.
///
/// ```
/// # use dungen_minion_geometry::*;
/// let north: OrdinalDirection = OrdinalDirection::North;
/// let east: OrdinalDirection = OrdinalDirection::East;
/// let south: OrdinalDirection = OrdinalDirection::South;
/// let west: OrdinalDirection = OrdinalDirection::West;
///
/// // Moving from north to east is a 90-degree rotation to the right.
/// assert!((east - north) == OrdinalRotation::Right90);
///
/// // Moving from north to south is a full 180-degree rotation.
/// assert!((south - north) == OrdinalRotation::Full180);
///
/// // Moving from north to west is a 90-degree rotation to the left.
/// assert!((west - north) == OrdinalRotation::Left90);
///
/// // Moving from north to north is defined as not a rotation, even if you rotated to get there.
/// assert!((north - north) == OrdinalRotation::None);
/// ```
#[derive(Copy, Clone, Debug, Display, Eq, Hash, PartialEq)]
pub enum OrdinalDirection {
    /// Represents a cartesian (0, +1) direction.
    North,
    /// Represents a cartesian (+1, 0) direction.
    East,
    /// Represents a cartesian (0, -1) direction.
    South,
    /// Represents a cartesian (-1, 0) direction.
    West,
}

impl From<i8> for OrdinalDirection {
    fn from(value: i8) -> Self {
        let mut value = value % 4;
        if value < 0 {
            value += value;
        }

        match value {
            0 => OrdinalDirection::North,
            1 => OrdinalDirection::East,
            2 => OrdinalDirection::South,
            3 => OrdinalDirection::West,
            _ => panic!("Wrapping and if-check should not allow this: {}", value),
        }
    }
}

impl From<OrdinalDirection> for i8 {
    fn from(value: OrdinalDirection) -> i8 {
        match value {
            OrdinalDirection::North => 0,
            OrdinalDirection::East => 1,
            OrdinalDirection::South => 2,
            OrdinalDirection::West => 3,
        }
    }
}

impl Add<OrdinalRotation> for OrdinalDirection {
    type Output = OrdinalDirection;

    fn add(self, other: OrdinalRotation) -> Self::Output {
        Self::from(i8::from(self) + i8::from(other))
    }
}

impl Sub<OrdinalDirection> for OrdinalDirection {
    type Output = OrdinalRotation;

    fn sub(self, other: OrdinalDirection) -> Self::Output {
        OrdinalRotation::from(i8::from(self) - i8::from(other))
    }
}
