// External includes.

// Standard includes.

// Internal includes.
use super::{HasSize, Length};

#[derive(Copy, Clone, Debug)]
pub struct Size {
    height: Length,
    width: Length,
}

impl Size {
    pub fn new(width: Length, height: Length) -> Self {
        Self { height, width }
    }

    pub fn height(&self) -> Length {
        self.height
    }

    pub fn height_mut(&mut self) -> &mut Length {
        &mut self.height
    }

    pub fn width(&self) -> Length {
        self.width
    }

    pub fn width_mut(&mut self) -> &mut Length {
        &mut self.width
    }
}

impl HasSize for Size {
    fn size(&self) -> &Self {
        self
    }

    fn size_mut(&mut self) -> &mut Self {
        self
    }
}
