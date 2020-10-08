// External includes.

// Standard includes.

// Internal includes.
use super::{Coord, HasPosition, HasSize, IsArea, IsPosition, IsSize, Length, Position, Size};

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

impl IsArea for Area {}

impl IsPosition for Area {
    fn x(&self) -> Coord {
        self.pos.x()
    }

    fn x_mut(&mut self) -> &mut Coord {
        self.pos.x_mut()
    }

    fn y(&self) -> Coord {
        self.pos.y()
    }

    fn y_mut(&mut self) -> &mut Coord {
        self.pos.y_mut()
    }
}

impl IsSize for Area {
    fn height(&self) -> Length {
        self.size.height()
    }

    fn height_mut(&mut self) -> &mut Length {
        self.size.height_mut()
    }

    fn width(&self) -> Length {
        self.size.width()
    }

    fn width_mut(&mut self) -> &mut Length {
        self.size.width_mut()
    }
}
