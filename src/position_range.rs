// External includes.
use rand::distributions::Distribution;
use rand::{thread_rng, Rng};

// Standard includes.

// Internal includes.
use super::{Coord, IsPosition, Position, ProvidesPosition, ProvidesShapePosition, ShapePosition};

/// Provides a range of [`Position`](struct.Position.html)s, from a start position to an end position.
///
/// Both of these methods provide a random position between the start position in the range, and the end position in the range. The x- and y-components of the returned `Position` are bounded together, such that the returned random position is somewhere along a tiled line from the start to the end position.
/// ```
/// # use dungen_minion_geometry::*;
/// use rand::{thread_rng, Rng};
/// // The end position is an inclusive bound.
/// // The divergent min and max for x and y guarantee that the samples are separate.
/// let position_range = PositionRange::new(Position::new(4, 14), Position::new(13, 23));
/// // Random generators are hard to guarantee. But this should be viable.
/// for _ in 0..5_000 {
///     let rand_position = position_range.provide_position();
///     assert!(rand_position.x() >= 4 && rand_position.x() <= 13);
///     assert!(rand_position.y() >= 14 && rand_position.y() <= 23);
/// }
///
/// let mut rng = thread_rng();
/// for _ in 0..5_000 {
///     let rand_position = rng.sample(position_range);
///     assert!(rand_position.x() >= 4 && rand_position.x() <= 13);
///     assert!(rand_position.y() >= 14 && rand_position.y() <= 23);
/// }
/// ```
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct PositionRange {
    start: Position,
    end: Position,
}

impl PositionRange {
    /// Creates a new `PositionRange` from a start `Position` and an end `Position`.
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

impl Distribution<Position> for PositionRange {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Position
    where
        R: Rng,
    {
        // rng.gen<f64>() generates the range [0.0, 1.0), and thus cannot generate 1.0.
        // This is sub-optimal for this use case.
        let i = rng.gen::<u64>() as f64 / 18_446_744_073_709_551_615.0f64;
        Position::new(
            (self.start.x() as f64 * (1.0 - i)).round() as Coord
                + (self.end.x() as f64 * i).round() as Coord,
            (self.start.y() as f64 * (1.0 - i)).round() as Coord
                + (self.end.y() as f64 * i).round() as Coord,
        )
    }
}

impl ProvidesPosition for PositionRange {
    fn provide_position(&self) -> Position {
        self.sample(&mut thread_rng())
    }
}

impl ProvidesShapePosition for PositionRange {
    /// Provides a range of [`ShapePosition`](struct.ShapePosition.html)s, from a start shape position to an end shape position.
    ///
    /// This method provide a random shape position between the start shape position in the range, and the end shape position in the range. The x- and y-components of the returned `ShapePosition` are bounded together, such that the returned random shape position is somewhere along a tiled line from the start to the end shape position.
    ///
    /// ShapePosition generation treats the start of the PositionRange as the (0, 0) position.
    /// ```
    /// # use dungen_minion_geometry::*;
    /// // The end position is an inclusive bound.
    /// // The divergent min and max for x and y guarantee that the samples are separate.
    /// let position_range = PositionRange::new(Position::new(4, 14), Position::new(13, 23));
    /// // Random generators are hard to guarantee. But this should be viable.
    /// for _ in 0..5_000 {
    ///     let rand_shape_position = position_range.provide_shape_position();
    ///     assert!(rand_shape_position.x() >= 0 && rand_shape_position.x() <= 9);
    ///     assert!(rand_shape_position.y() >= 0 && rand_shape_position.y() <= 19);
    /// }
    /// ```
    fn provide_shape_position(&self) -> ShapePosition {
        let position = self.provide_position() - self.start;
        ShapePosition::new(position.x(), position.x())
    }
}