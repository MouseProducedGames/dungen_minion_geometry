// External includes.

// Standard includes.
use std::convert::From;
use std::ops::{Add, Neg, Sub};

// Internal includes.

/// Defines rotation on a cartesian plane where each rotation is constructed out of 90-degree angles.
///
/// ```
/// # use dungen_minion_geometry::*;
/// let rotation_none: CardinalRotation = CardinalRotation::None;
///
/// let rotation_right90: CardinalRotation = rotation_none + CardinalRotation::Right90;
/// assert!(rotation_right90 == CardinalRotation::Right90);
///
/// let rotation_full180: CardinalRotation = rotation_right90 + CardinalRotation::Right90;
/// assert!(rotation_full180 == CardinalRotation::Full180);
///
/// let rotation_left90: CardinalRotation = rotation_full180 + CardinalRotation::Right90;
/// assert!(rotation_left90 == CardinalRotation::Left90);
///
/// let rotation_back_to_none: CardinalRotation = rotation_left90 + CardinalRotation::Right90;
/// assert!(rotation_back_to_none== CardinalRotation::None);
/// ```
#[derive(Copy, Clone, Debug, Display, Eq, Hash, PartialEq)]
pub enum CardinalRotation {
    /// No rotation; provided for the sake of completness.
    None,
    /// Rotate 90 degrees to the right on a 2D cartesian plane.
    Right90,
    /// Complete a full 180-degree rotation on a 2D cartesian plane. This rotation is not defined as left or right.
    Full180,
    /// Rotate 90 degrees to the left on a 2D cartesian plane.
    Left90,
}

impl From<i8> for CardinalRotation {
    fn from(value: i8) -> Self {
        let mut value = value % 4;
        if value < 0 {
            value = 4 - value.abs();
        }

        match value {
            0 => CardinalRotation::None,
            1 => CardinalRotation::Right90,
            2 => CardinalRotation::Full180,
            3 => CardinalRotation::Left90,
            _ => panic!("Wrapping and if-check should not allow this: {}", value),
        }
    }
}

impl From<CardinalRotation> for i8 {
    fn from(value: CardinalRotation) -> i8 {
        match value {
            CardinalRotation::None => 0,
            CardinalRotation::Right90 => 1,
            CardinalRotation::Full180 => 2,
            CardinalRotation::Left90 => 3,
        }
    }
}

impl Add<CardinalRotation> for CardinalRotation {
    type Output = CardinalRotation;

    fn add(self, other: CardinalRotation) -> Self::Output {
        Self::from(i8::from(self) + i8::from(other))
    }
}

impl Neg for CardinalRotation {
    type Output = CardinalRotation;

    fn neg(self) -> Self::Output {
        CardinalRotation::from(i8::from(self) + 2)
    }
}

impl Sub<CardinalRotation> for CardinalRotation {
    type Output = CardinalRotation;

    fn sub(self, other: CardinalRotation) -> Self::Output {
        Self::from(i8::from(self) - i8::from(other))
    }
}
