// External includes.
use rand::distributions::Distribution;
use rand::{thread_rng, Rng};

// Standard includes.

// Internal includes.
use super::{IsSize, ProvidesSize, Size};

/// Provides a range of [`Size`](struct.Size.html)s, from a minimum size to a maximum size.
///
/// Both of these methods provide a random size between the minimum size in the range, and the maximum size in the range. The width and height of the returned `Size` are bounded separately.
/// ```
/// # use dungen_minion_geometry::*;
/// use rand::{thread_rng, Rng};
/// // The maximum size is an exclusive bound.
/// // The divergent min and max for width and height guarantee that the samples are separate.
/// let size_range = SizeRange::new(Size::new(4, 14), Size::new(13, 23));
/// // Random generators are hard to guarantee. But this should be viable.
/// for _ in 0..5_000 {
///     let rand_size = size_range.provide_size();
///     assert!(rand_size.width() >= 4 && rand_size.width() < 13);
///     assert!(rand_size.height() >= 14 && rand_size.height() < 23);
/// }
///
/// let mut rng = thread_rng();
/// for _ in 0..5_000 {
///     let rand_size = rng.sample(size_range);
///     assert!(rand_size.width() >= 4 && rand_size.width() < 13);
///     assert!(rand_size.height() >= 14 && rand_size.height() < 23);
/// }
/// ```
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct SizeRange {
    min_size: Size,
    max_size: Size,
}

impl SizeRange {
    /// Creates a new `SizeRange` from a minimum `Size` and a maximum `Size`.
    pub fn new(min_size: Size, max_size: Size) -> Self {
        Self { min_size, max_size }
    }
}

impl Distribution<Size> for SizeRange {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Size
    where
        R: Rng,
    {
        Size::new(
            rng.gen_range(self.min_size.width(), self.max_size.width()),
            rng.gen_range(self.min_size.height(), self.max_size.height()),
        )
    }
}

impl ProvidesSize for SizeRange {
    fn provide_size(&self) -> Size {
        self.sample(&mut thread_rng())
    }
}