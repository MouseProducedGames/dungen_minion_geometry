// External includes.
use rand::distributions::Distribution;
use rand::{thread_rng, Rng};

// Standard includes.

// Internal includes.
use super::{Count, ProvidesCount};

/// Provides a range of [`Count`](type.Count.html)s, from a minimum count to a maximum count.
///
/// Both of these methods provide a random count between the minimum count in the range, and the maximum count in the range.
/// ```
/// # use dungen_minion_geometry::*;
/// use rand::{thread_rng, Rng};
/// // The maximum count is an inclusive bound.
/// let count_range = CountRange::new(4, 13);
/// // Random generators are hard to guarantee. But this should be viable.
/// for _ in 0..5_000 {
///     let rand_count = count_range.provide_count();
///     assert!(rand_count >= 4 && rand_count <= 13);
/// }
///
/// let mut rng = thread_rng();
/// for _ in 0..5_000 {
///     let rand_count = rng.sample(count_range);
///     assert!(rand_count >= 4 && rand_count <= 13);
/// }
/// ```
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct CountRange {
    min_count: Count,
    max_count: Count,
}

impl CountRange {
    /// Creates a new `SizeRange` from a minimum `Size` and a maximum `Size`.
    pub fn new(min_count: Count, max_count: Count) -> Self {
        Self {
            min_count,
            max_count,
        }
    }
}

impl Distribution<Count> for CountRange {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Count
    where
        R: Rng,
    {
        rng.gen_range(self.min_count, self.max_count + 1)
    }
}

impl ProvidesCount for CountRange {
    fn provide_count(&self) -> Count {
        self.sample(&mut thread_rng())
    }
}
