// External includes.

// Standard includes.

// Internal includes.
use super::{IsSize, Length};

#[derive(Copy, Clone, Debug)]
pub struct Size {
    height: Length,
    width: Length,
}

impl Size {
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
