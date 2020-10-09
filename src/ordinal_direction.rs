// External includes.

// Standard includes.
use std::convert::From;
use std::ops::{Add, Sub};

// Internal includes.
use super::OrdinalRotation;

#[derive(Copy, Clone, Debug, Display)]
pub enum OrdinalDirection {
    North,
    East,
    South,
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
