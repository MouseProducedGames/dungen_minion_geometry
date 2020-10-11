// External includes.
use rand::distributions::Distribution;
use rand::{thread_rng, Rng};

// Standard includes.

// Internal includes.
use super::{
    HasShapePosition, HasSize, ProvidesShapeArea, ProvidesShapePosition, ProvidesSize, ShapeArea,
    ShapePosition, ShapePositionRange, Size, SizeRange,
};

/// Provides a range of [`ShapeArea`](struct.ShapeArea.html)s, from a minimum shape area to a maximum shape area.
///
/// Both of these methods provide a random shape area between the minimum shape area in the range, and the maximum shape area in the range. The x- and y-components of the returned `ShapeArea` are bounded together, such that the returned random shape position is somewhere along a tiled line from the start to the end position. The width, and height, of the returned `ShapeArea` are bounded separately.
/// ```
/// # use dungen_minion_geometry::*;
/// use rand::{thread_rng, Rng};
/// // The end shape position is an inclusive bound.
/// // The maximum size is an inclusive bound.
/// // The divergent min and max for shape position and size guarantee that the samples are separate.
/// let shape_area_range = ShapeAreaRange::new(
///     ShapePositionRange::new(ShapePosition::new(4, 14), ShapePosition::new(13, 23)),
///     SizeRange::new(Size::new(24, 34), Size::new(33, 43)),
///     );
/// // Random generators are hard to guarantee. But this should be viable.
/// for _ in 0..5_000 {
///     let rand_shape_area = shape_area_range.provide_shape_area();
///     assert!(rand_shape_area.shape_position().x() >= 4);
///     assert!(rand_shape_area.shape_position().x() <= 13);
///     assert!(rand_shape_area.shape_position().y() >= 14);
///     assert!(rand_shape_area.shape_position().y() <= 23);
///     assert!(rand_shape_area.size().width() >= 24 && rand_shape_area.size().width() <= 33);
///     assert!(rand_shape_area.size().height() >= 34 && rand_shape_area.size().height() <= 43);
/// }
///
/// let mut rng = thread_rng();
/// for _ in 0..5_000 {
///     let rand_shape_area = rng.sample(shape_area_range);
///     assert!(rand_shape_area.shape_position().x() >= 4);
///     assert!(rand_shape_area.shape_position().x() <= 13);
///     assert!(rand_shape_area.shape_position().y() >= 14);
///     assert!(rand_shape_area.shape_position().y() <= 23);
///     assert!(rand_shape_area.size().width() >= 24 && rand_shape_area.size().width() <= 33);
///     assert!(rand_shape_area.size().height() >= 34 && rand_shape_area.size().height() <= 43);
/// }
/// ```
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct ShapeAreaRange {
    shape_position_range: ShapePositionRange,
    size_range: SizeRange,
}

impl ShapeAreaRange {
    /// Creates a new `ShapeAreaRange` from a `ShapePositionRange` and `SizeRange`.
    pub fn new(shape_position_range: ShapePositionRange, size_range: SizeRange) -> Self {
        Self {
            shape_position_range,
            size_range,
        }
    }
}

impl Distribution<ShapeArea> for ShapeAreaRange {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ShapeArea
    where
        R: Rng,
    {
        ShapeArea::new(
            rng.sample(self.shape_position_range),
            rng.sample(self.size_range),
        )
    }
}

impl From<ShapeArea> for ShapeAreaRange {
    fn from(shape_area: ShapeArea) -> Self {
        Self::new(
            ShapePositionRange::from(*shape_area.shape_position()),
            SizeRange::from(*shape_area.size()),
        )
    }
}

impl From<Size> for ShapeAreaRange {
    fn from(size: Size) -> Self {
        Self::new(
            ShapePositionRange::from(ShapePosition::new(0, 0)),
            SizeRange::from(size),
        )
    }
}

impl ProvidesShapeArea for ShapeAreaRange {
    fn provide_shape_area(&self) -> ShapeArea {
        self.sample(&mut thread_rng())
    }
}

impl ProvidesShapePosition for ShapeAreaRange {
    fn provide_shape_position(&self) -> ShapePosition {
        self.shape_position_range.sample(&mut thread_rng())
    }
}

impl ProvidesSize for ShapeAreaRange {
    fn provide_size(&self) -> Size {
        self.size_range.sample(&mut thread_rng())
    }
}
