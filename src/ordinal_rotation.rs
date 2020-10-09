// External includes.

// Standard includes.
use std::convert::From;
use std::ops::{Add, Sub};

// Internal includes.

/// Defines rotation on a cartesian plane where each rotation is constructed out of 90-degree angles.
///
/// ```
/// # use dungen_minion_geometry::*;
/// let rotation_none: OrdinalRotation = OrdinalRotation::None;
/// 
/// let rotation_right90: OrdinalRotation = rotation_none + OrdinalRotation::Right90;
/// assert!(rotation_right90 == OrdinalRotation::Right90);
/// 
/// let rotation_full180: OrdinalRotation = rotation_right90 + OrdinalRotation::Right90;
/// assert!(rotation_full180 == OrdinalRotation::Full180);
/// 
/// let rotation_left90: OrdinalRotation = rotation_full180 + OrdinalRotation::Right90;
/// assert!(rotation_left90 == OrdinalRotation::Left90);
/// 
/// let rotation_back_to_none: OrdinalRotation = rotation_left90 + OrdinalRotation::Right90;
/// assert!(rotation_back_to_none== OrdinalRotation::None);
/// ```
#[derive(Copy, Clone, Debug, Display, Eq, PartialEq)]
pub enum OrdinalRotation {
    /// No rotation; provided for the sake of completness.
    None,
    /// Rotate 90 degrees to the right on a 2D cartesian plane.
    Right90,
    /// Complete a full 180-degree rotation on a 2D cartesian plane. This rotation is not defined as left or right.
    Full180,
    /// Rotate 90 degrees to the left on a 2D cartesian plane.
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
