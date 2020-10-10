// External includes.
use rand::distributions::Distribution;
use rand::{thread_rng, Rng};

// Standard includes.

// Internal includes.
use super::{Coord, IsShapePosition, ProvidesShapePosition, ShapePosition};

/// Provides a range of [`ShapePosition`](struct.ShapePosition.html)s, from a start shape position to an end shape position.
///
/// Both of these methods provide a random shape position between the start shape position in the range, and the end shape position in the range. The x- and y-components of the returned `ShapePosition` are bounded together, such that the returned random shape position is somewhere along a tiled line from the start to the end shape position.
/// ```
/// # use dungen_minion_geometry::*;
/// use rand::{thread_rng, Rng};
/// // The end shape position is an inclusive bound.
/// // The divergent min and max for x and y guarantee that the samples are separate.
/// let shape_position_range = ShapePositionRange::new(
///     ShapePosition::new(4, 14), ShapePosition::new(13, 23)
///     );
/// // Random generators are hard to guarantee. But this should be viable.
/// for _ in 0..5_000 {
///     let rand_shape_position = shape_position_range.provide_shape_position();
///     assert!(rand_shape_position.x() >= 4 && rand_shape_position.x() <= 13);
///     assert!(rand_shape_position.y() >= 14 && rand_shape_position.y() <= 23);
/// }
///
/// let mut rng = thread_rng();
/// for _ in 0..5_000 {
///     let rand_shape_position = rng.sample(shape_position_range);
///     assert!(rand_shape_position.x() >= 4 && rand_shape_position.x() <= 13);
///     assert!(rand_shape_position.y() >= 14 && rand_shape_position.y() <= 23);
/// }
/// ```
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct ShapePositionRange {
    start: ShapePosition,
    end: ShapePosition,
}

impl ShapePositionRange {
    /// Creates a new `ShapePositionRange` from a start `ShapePosition` and an end `ShapePosition`.
    pub fn new(start: ShapePosition, end: ShapePosition) -> Self {
        Self { start, end }
    }
}

impl Distribution<ShapePosition> for ShapePositionRange {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ShapePosition
    where
        R: Rng,
    {
        // rng.gen<f64>() generates the range [0.0, 1.0), and thus cannot generate 1.0.
        // This is sub-optimal for this use case.
        let i = rng.gen::<u64>() as f64 / 18_446_744_073_709_551_615.0f64;
        ShapePosition::new(
            (self.start.x() as f64 * (1.0 - i)).round() as Coord
                + (self.end.x() as f64 * i).round() as Coord,
            (self.start.y() as f64 * (1.0 - i)).round() as Coord
                + (self.end.y() as f64 * i).round() as Coord,
        )
    }
}

impl ProvidesShapePosition for ShapePositionRange {
    fn provide_shape_position(&self) -> ShapePosition {
        self.sample(&mut thread_rng())
    }
}
