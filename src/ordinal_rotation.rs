// External includes.

// Standard includes.
use std::convert::From;
use std::ops::{Add, Sub};

// Internal includes.

#[derive(Copy, Clone, Debug, Display)]
pub enum OrdinalRotation {
    None,
    Right90,
    Full180,
    Left90,
}

impl From<i8> for OrdinalRotation {
    fn from(value: i8) -> Self {
        let mut value = value % 4;
        if value < 0 {
            value += value;
        }

        match value {
            0 => OrdinalRotation::None,
            1 => OrdinalRotation::Right90,
            2 => OrdinalRotation::Full180,
            3 => OrdinalRotation::Left90,
            _ => panic!("Wrapping and if-check should not allow this: {}", value),
        }
    }
}

impl From<OrdinalRotation> for i8 {
    fn from(value: OrdinalRotation) -> i8 {
        match value {
            OrdinalRotation::None => 0,
            OrdinalRotation::Right90 => 1,
            OrdinalRotation::Full180 => 2,
            OrdinalRotation::Left90 => 3,
        }
    }
}

impl Add<OrdinalRotation> for OrdinalRotation {
    type Output = OrdinalRotation;

    fn add(self, other: OrdinalRotation) -> Self::Output {
        Self::from(i8::from(self) + i8::from(other))
    }
}

impl Sub<OrdinalRotation> for OrdinalRotation {
    type Output = OrdinalRotation;

    fn sub(self, other: OrdinalRotation) -> Self::Output {
        Self::from(i8::from(self) - i8::from(other))
    }
}
