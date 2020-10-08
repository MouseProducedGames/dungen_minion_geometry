// External includes.

// Standard includes.

// Internal includes.
use super::{HasArea, HasPosition, HasSize, Position, Size};

#[derive(Copy, Clone, Debug)]
pub struct Area {
    pos: Position,
    size: Size,
}

impl Area {
    pub fn new(pos: Position, size: Size) -> Self {
        Self { pos, size }
    }
}

impl HasArea for Area {
    fn area(&self) -> &Self {
        self
    }

    fn area_mut(&mut self) -> &mut Self {
        self
    }
}

impl HasPosition for Area {
    fn pos(&self) -> &Position {
        &self.pos
    }

    fn pos_mut(&mut self) -> &mut Position {
        &mut self.pos
    }
}

impl HasSize for Area {
    fn size(&self) -> &Size {
        &self.size
    }

    fn size_mut(&mut self) -> &mut Size {
        &mut self.size
    }
}
