// External includes.
use rand::distributions::Distribution;
use rand::{thread_rng, Rng};

// Standard includes.

// Internal includes.
use super::{
    Area, Coord, IsArea, Position, PositionRange, ProvidesArea, ProvidesPosition,
    ProvidesShapePosition, ProvidesSize, ShapePosition, Size, SizeRange,
};

/// Provides a range of [`Area`](struct.Area.html)s, from a minimum area to a maximum area.
///
/// Both of these methods provide a random area between the minimum area in the range, and the maximum area in the range. The x- and y-components of the returned `Area` are bounded together, such that the returned random position is somewhere along a tiled line from the start to the end position. The width, and height, of the returned `Area` are bounded separately.
/// ```
/// # use dungen_minion_geometry::*;
/// use rand::{thread_rng, Rng};
/// // The end position is an inclusive bound.
/// // The maximum size is an inclusive bound.
/// // The divergent min and max for position and size guarantee that the samples are separate.
/// let area_range = AreaRange::new(
///     PositionRange::new(Position::new(4, 14), Position::new(13, 23)),
///     SizeRange::new(Size::new(24, 34), Size::new(33, 43)),
///     );
/// // Random generators are hard to guarantee. But this should be viable.
/// for _ in 0..5_000 {
///     let rand_area = area_range.provide_area();
///     assert!(rand_area.position().x() >= 4 && rand_area.position().x() <= 13);
///     assert!(rand_area.position().y() >= 14 && rand_area.position().y() <= 23);
///     assert!(rand_area.size().width() >= 24 && rand_area.size().width() <= 33);
///     assert!(rand_area.size().height() >= 34 && rand_area.size().height() <= 43);
/// }
///
/// let mut rng = thread_rng();
/// for _ in 0..5_000 {
///     let rand_area = rng.sample(area_range);
///     assert!(rand_area.position().x() >= 4 && rand_area.position().x() <= 13);
///     assert!(rand_area.position().y() >= 14 && rand_area.position().y() <= 23);
///     assert!(rand_area.size().width() >= 24 && rand_area.size().width() <= 33);
///     assert!(rand_area.size().height() >= 34 && rand_area.size().height() <= 43);
/// }
/// ```
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct AreaRange {
    position_range: PositionRange,
    size_range: SizeRange,
}

impl AreaRange {
    /// Creates a new `SizeRange` from a minimum `Size` and a maximum `Size`.
    pub fn new(position_range: PositionRange, size_range: SizeRange) -> Self {
        Self {
            position_range,
            size_range,
        }
    }
}

impl Distribution<Area> for AreaRange {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Area
    where
        R: Rng,
    {
        Area::new(rng.sample(self.position_range), rng.sample(self.size_range))
    }
}

impl ProvidesArea for AreaRange {
    fn provide_area(&self) -> Area {
        self.sample(&mut thread_rng())
    }
}
