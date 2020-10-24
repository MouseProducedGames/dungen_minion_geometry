// External includes.

// Standard includes.

// Internal includes.
use super::{
    Area, Containment, ContainsLocalPosition, ContainsPosition, Coord, HasArea, HasBottom,
    HasPosition, HasRight, HasSize, Inclusion, IntersectsLocalPosition, IntersectsPosition, IsArea,
    IsSize, Placed, PlacedObject, PlacedShape, Position, ProvidesArea, ProvidesPlacedShape,
    ProvidesSize, Shape, Size,
};

/// Contains a slice of [`PlacedShape`](trait.PlacedShape.html) and implements `PlacedShape` for the collection.
///
/// Containment is based on the union of the various `PlacedShape`s.
/// ```
/// # use dungen_minion_geometry::*;
///
/// let values: Box<[(Inclusion, Box<dyn PlacedShape>); 2]> = Box::new([
///     (Inclusion::Include, Box::new(Area::new(Position::new(1, 0), Size::new(3, 5)))),
///     (Inclusion::Include, Box::new(Area::new(Position::new(0, 1), Size::new(5, 3)))),
/// ]);
/// let its_a_big_plus = PlacedShapeSlice::new(values);
///
/// let mut intersection = Vec::new();
/// let check = [
///     [false, true, true, true, false],
///     [true, true, true, true, true],
///     [true, true, true, true, true],
///     [true, true, true, true, true],
///     [false, true, true, true, false],
/// ];
/// for y in 0..5 {
///     intersection.clear();
///     for x in 0..5 {
///         intersection.push(its_a_big_plus.intersects_position(Position::new(x, y)));
///     }
///     println!("{}: {:?}", y, intersection);
///     assert!(intersection == check[y as usize]);
/// }
/// ```
#[derive(Clone)]
pub struct PlacedShapeSlice {
    area: Area,
    values: Box<[(Inclusion, Box<dyn PlacedShape>)]>,
}

impl PlacedShapeSlice {
    /// Creates a new `PlacedShapeSlice`.
    pub fn new(values: Box<[(Inclusion, Box<dyn PlacedShape>)]>) -> Self {
        if values.is_empty() {
            Self {
                area: Area::new(Position::zero(), Size::zero()),
                values,
            }
        } else {
            let mut left = Coord::MAX;
            let mut top = Coord::MAX;
            let mut right = Coord::MIN;
            let mut bottom = Coord::MIN;
            for value in values.iter() {
                left = left.min(value.1.left());
                top = top.min(value.1.top());
                right = right.max(value.1.right());
                bottom = bottom.max(value.1.bottom());
            }
            let mut area = Area::new(Position::new(left, top), Size::zero());
            area.right_set(right);
            area.bottom_set(bottom);
            Self { area, values }
        }
    }
}

impl ContainsLocalPosition for PlacedShapeSlice {
    fn contains_local_position(&self, position: Position) -> Containment {
        self.contains_position(*self.position() + position)
    }
}

impl ContainsPosition for PlacedShapeSlice {
    fn contains_position(&self, position: Position) -> Containment {
        if self.values.is_empty() {
            Containment::Disjoint
        } else {
            let mut containment = Containment::Disjoint;
            for value in self.values.iter() {
                containment = match value.0 {
                    Inclusion::Include => containment.max(value.1.contains_position(position)),
                    Inclusion::Exclude => containment.min(value.1.contains_position(position)),
                };
            }

            containment
        }
    }
}

impl HasArea for PlacedShapeSlice {
    fn area(&self) -> &Area {
        &self.area
    }

    fn area_mut(&mut self) -> &mut Area {
        &mut self.area
    }
}

impl HasPosition for PlacedShapeSlice {
    fn position(&self) -> &Position {
        self.area().position()
    }

    fn position_mut(&mut self) -> &mut Position {
        self.area_mut().position_mut()
    }
}

impl HasSize for PlacedShapeSlice {
    fn size(&self) -> &Size {
        self.area().size()
    }

    fn size_mut(&mut self) -> &mut Size {
        self.area_mut().size_mut()
    }
}

impl IntersectsLocalPosition for PlacedShapeSlice {
    fn intersects_local_position(&self, position: Position) -> bool {
        self.intersects_position(*self.position() + position)
    }
}

impl IntersectsPosition for PlacedShapeSlice {
    fn intersects_position(&self, position: Position) -> bool {
        self.contains_position(position) != Containment::Disjoint
    }
}

impl IsArea for PlacedShapeSlice {}

impl IsSize for PlacedShapeSlice {}

impl Placed for PlacedShapeSlice {}

impl PlacedObject for PlacedShapeSlice {}

impl ProvidesArea for PlacedShapeSlice {
    fn provide_area(&self) -> Area {
        *self.area()
    }
}

impl ProvidesPlacedShape for PlacedShapeSlice {
    fn provide_placed_shape(&self) -> Box<dyn PlacedShape> {
        Box::new(self.clone())
    }
}

impl ProvidesSize for PlacedShapeSlice {
    fn provide_size(&self) -> Size {
        *self.area.size()
    }
}

impl Shape for PlacedShapeSlice {
    fn box_shape_clone(&self) -> Box<dyn Shape> {
        Box::new((*self).clone())
    }
}
