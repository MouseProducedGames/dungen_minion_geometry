// External includes.
use rand::distributions::Distribution;
use rand::{thread_rng, Rng};

// Standard includes.

// Internal includes.
use super::{
    Area, HasPosition, HasSize, PlacedShape, Position, PositionRange, ProvidesArea,
    ProvidesPlacedShape, ProvidesPosition, ProvidesSize, Size, SizeRange,
};

/// Provides a range of [`Area`](struct.Area.html)s, from a minimum area to a maximum area.
///
/// Both of these methods provide a random area between the minimum area in the range, and the maximum area in the range. The x- and y-components of the returned `Area` are bounded together, such that the returned random position is somewhere along a tiled line from the start to the end position. The width, and height, of the returned `Area` are bounded separately.
/// ```
/// # use dungen_minion_geometry::*;
/// use std::sync::Arc;
/// use rayon::prelude::*;
///
/// use rand::{thread_rng, Rng};
/// // The end position is an inclusive bound.
/// // The maximum size is an inclusive bound.
/// // The divergent min and max for position and size guarantee that the samples are separate.
/// let area_range = Arc::new(AreaRange::new(
///     PositionRange::new(Position::new(4, 14), Position::new(13, 23)),
///     SizeRange::new(Size::new(24, 34), Size::new(33, 43)),
///     ));
/// // Random generators are hard to guarantee. But this should be viable.
/// [0..5_000].par_iter().for_each(|_i| {
///     let rand_area = area_range.provide_area();
///     assert!(rand_area.position().x() >= 4 && rand_area.position().x() <= 13);
///     assert!(rand_area.position().y() >= 14 && rand_area.position().y() <= 23);
///     assert!(rand_area.size().width() >= 24 && rand_area.size().width() <= 33);
///     assert!(rand_area.size().height() >= 34 && rand_area.size().height() <= 43);
/// });
///
/// [0..5_000].par_iter().for_each(|_i| {
///     let rand_area = thread_rng().sample(*area_range);
///     assert!(rand_area.position().x() >= 4 && rand_area.position().x() <= 13);
///     assert!(rand_area.position().y() >= 14 && rand_area.position().y() <= 23);
///     assert!(rand_area.size().width() >= 24 && rand_area.size().width() <= 33);
///     assert!(rand_area.size().height() >= 34 && rand_area.size().height() <= 43);
/// });
/// ```
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct AreaRange {
    position_range: PositionRange,
    size_range: SizeRange,
}

impl AreaRange {
    /// Creates a new `AreaRange` from a `PositionRange` and `SizeRange`.
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

impl From<Area> for AreaRange {
    fn from(area: Area) -> Self {
        AreaRange::new(
            PositionRange::from(*area.position()),
            SizeRange::from(*area.size()),
        )
    }
}

impl ProvidesArea for AreaRange {
    fn provide_area(&self) -> Area {
        self.sample(&mut thread_rng())
    }
}

impl ProvidesPlacedShape for AreaRange {
    fn provide_placed_shape(&self) -> Box<dyn PlacedShape> {
        Box::new(self.provide_area())
    }
}

impl ProvidesPosition for AreaRange {
    fn provide_position(&self) -> Position {
        self.position_range.sample(&mut thread_rng())
    }
}

impl ProvidesSize for AreaRange {
    fn provide_size(&self) -> Size {
        self.size_range.sample(&mut thread_rng())
    }
}
