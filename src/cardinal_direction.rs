// External includes.

// Standard includes.
use std::convert::From;
use std::ops::{Add, AddAssign, Neg, Sub};

// Internal includes.
use super::{CardinalRotation, IsPosition, Position};

/// Defines a direction on a cartesian plane where each direction is an orthogonal, and cardinal, 90-degree vector.
///
/// ```
/// # use dungen_minion_geometry::*;
/// let north: CardinalDirection = CardinalDirection::North;
/// let east: CardinalDirection = CardinalDirection::East;
/// let south: CardinalDirection = CardinalDirection::South;
/// let west: CardinalDirection = CardinalDirection::West;
///
/// // Moving from north to east is a 90-degree rotation to the right.
/// assert!((east - north) == CardinalRotation::Right90);
///
/// // Moving from north to south is a full 180-degree rotation.
/// assert!((south - north) == CardinalRotation::Full180);
///
/// // Moving from north to west is a 90-degree rotation to the left.
/// assert!((west - north) == CardinalRotation::Left90);
///
/// // Moving from north to north is defined as not a rotation, even if you rotated to get there.
/// assert!((north - north) == CardinalRotation::None);
/// ```
#[derive(Copy, Clone, Debug, Display, Eq, Hash, PartialEq)]
pub enum CardinalDirection {
    /// Represents a cartesian (0, +1) direction.
    North,
    /// Represents a cartesian (+1, 0) direction.
    East,
    /// Represents a cartesian (0, -1) direction.
    South,
    /// Represents a cartesian (-1, 0) direction.
    West,
}

impl Add<CardinalRotation> for CardinalDirection {
    type Output = CardinalDirection;

    fn add(self, other: CardinalRotation) -> Self::Output {
        Self::from(i8::from(self) + i8::from(other))
    }
}

impl AddAssign<CardinalRotation> for CardinalDirection {
    fn add_assign(&mut self, other: CardinalRotation) {
        *self = *self + other
    }
}

impl From<i8> for CardinalDirection {
    fn from(value: i8) -> Self {
        let mut value = value % 4;
        if value < 0 {
            value = 4 - value.abs();
        }

        match value {
            0 => CardinalDirection::North,
            1 => CardinalDirection::East,
            2 => CardinalDirection::South,
            3 => CardinalDirection::West,
            _ => panic!("Wrapping and if-check should not allow this: {}", value),
        }
    }
}

impl From<CardinalDirection> for i8 {
    fn from(value: CardinalDirection) -> i8 {
        match value {
            CardinalDirection::North => 0,
            CardinalDirection::East => 1,
            CardinalDirection::South => 2,
            CardinalDirection::West => 3,
        }
    }
}

impl From<Position> for Option<CardinalDirection> {
    /// Attempts to convert from a position offset to a CardinalDirection.
    ///
    /// ```
    /// use dungen_minion_geometry::*;
    /// assert!(Some(CardinalDirection::North) == Option::<CardinalDirection>::from(Position::new(0, -1)));
    /// assert!(None == Option::<CardinalDirection>::from(Position::new(1, -1)));
    /// assert!(Some(CardinalDirection::East) == Option::<CardinalDirection>::from(Position::new(1, 0)));
    /// assert!(None == Option::<CardinalDirection>::from(Position::new(1, 1)));
    /// assert!(Some(CardinalDirection::South) == Option::<CardinalDirection>::from(Position::new(0, 1)));
    /// assert!(None == Option::<CardinalDirection>::from(Position::new(-1, 1)));
    /// assert!(Some(CardinalDirection::West) == Option::<CardinalDirection>::from(Position::new(-1, 0)));
    /// assert!(None == Option::<CardinalDirection>::from(Position::new(-1, -1)));
    /// ```
    fn from(value: Position) -> Self {
        let index = match value.x().signum() {
            -1 => 1,
            1 => 2,
            // The only other possibility is zero, but Rust doesn't know that (21-10-2020 DD-MM-YYYY).
            _ => 0,
        } + match value.y().signum() {
            -1 => 4,
            1 => 8,
            // The only other possibility is zero, but Rust doesn't know that (21-10-2020 DD-MM-YYYY).
            _ => 0,
        };

        match index {
            1 => Some(CardinalDirection::West),
            2 => Some(CardinalDirection::East),
            4 => Some(CardinalDirection::North),
            8 => Some(CardinalDirection::South),
            _ => None,
        }
    }
}

impl Neg for CardinalDirection {
    type Output = CardinalDirection;

    fn neg(self) -> Self::Output {
        CardinalDirection::from(i8::from(self) + 2)
    }
}

impl Sub<CardinalDirection> for CardinalDirection {
    type Output = CardinalRotation;

    fn sub(self, other: CardinalDirection) -> Self::Output {
        CardinalRotation::from(i8::from(self) - i8::from(other))
    }
}
