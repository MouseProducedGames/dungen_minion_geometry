// External includes.

// Standard includes.

// Internal includes.
use super::{IsSize, Length};

/// Defines a `Size` with the given height and width, in [`Length`](type.Length.html) units.
/// 
/// Size does not have a position, and its width and height can each be an integer zero, or positive value. In the cartesian system used in dungen_minion, and most roguelikes, (x: 0, y: 0) defines the top-left of the coordinate system. 
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Size {
    height: Length,
    width: Length,
}

impl Size {
    /// Creates a new `Size` with the given height and width.
    ///
    /// ```
    /// # use dungen_minion_geometry::*;
    /// let value: Size = Size::new(42, 24);
    /// assert!(value.width() == 42);
    /// assert!(value.height() == 24);
    /// ```
    pub fn new(width: Length, height: Length) -> Self {
        Self { height, width }
    }
}

impl IsSize for Size {
    fn height(&self) -> Length {
        self.height
    }

    fn height_mut(&mut self) -> &mut Length {
        &mut self.height
    }

    fn width(&self) -> Length {
        self.width
    }

    fn width_mut(&mut self) -> &mut Length {
        &mut self.width
    }
}
