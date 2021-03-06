// External includes.

// Standard includes.

// Internal includes.
use super::{
    Area, Containment, ContainsLocalPosition, ContainsPosition, HasArea, HasHeight, HasPosition,
    HasSize, HasWidth, IntersectsLocalPosition, IntersectsPosition, IsPosition, Length, Placed,
    PlacedObject, PlacedShape, Position, ProvidesArea, ProvidesPlacedShape, ProvidesSize, Shape,
    Size,
};

/// Defines an oval by a [`Position`](struct.Position.html) and [`Size`](struct.Size.html).
///
///
/// The `Position` of the oval is the top-left corner of the rectangle surrounding the oval, and the `Size` of the oval determines the bottom-right corner of the rectangle surrounding the oval.
///
/// The oval is calculated from these extents.
/// ```
/// # use dungen_minion_geometry::*;
///
/// use rayon::prelude::*;
///
/// use std::collections::{HashSet, VecDeque};
///
/// fn draw_oval(oval: Oval, visited: &HashSet<Position>) {
///     for y in oval.top()..=oval.bottom() {
///         for x in oval.left()..=oval.right() {
///             let position = Position::new(x, y);
///             let draw_ch = if visited.contains(&position) {
///                 '*'
///             } else {
///                 match oval.contains_local_position(position) {
///                     Containment::Disjoint => ' ',
///                     Containment::Intersects => '#',
///                     Containment::Contains => '.',
///                 }
///             };
///             print!("{}", draw_ch);
///         }
///         println!();
///     }
/// }
///
/// fn test_oval(oval: Oval) {
///     // Perform a flood-fill to ensure the oval is bounded.
///     let mut queue = VecDeque::new();
///     let mut visited = HashSet::new();
///
///     // An oval contains a rectangular area which is guaranteed to be entirely inside that oval.
///     // Our starting positions are on the outside of that rectangle, and the rectangles'
///     // interior can be marked as visited.
///     let fwidth = ((oval.area().width() as f64 / 2.0) - 0.5).max(0.0);
///     let fheight = ((oval.area().height() as f64 / 2.0) - 0.5).max(0.0);
///     let fmin_bounds = (fwidth.min(fheight) - 1.0).max(0.0);
///     let fmin_bounds = ((fmin_bounds * fmin_bounds) / 2.0);
///     let fmin_bounds = if fmin_bounds > 0.0 {
///         fmin_bounds.sqrt()
///     } else {
///         fmin_bounds
///     };
///     let flocal_center_x = fwidth;
///     let flocal_center_y = fheight;
///     // let pos_x = oval.left() + flocal_center_x as Coord;
///     // let pos_y = oval.top() + flocal_center_y as Coord;
///     // queue.push_back(Position::new(pos_x, pos_y));
///     for y in oval.top()..=oval.bottom() {
///         let fy = y as f64;
///         let adjusted_position_y = (fy - flocal_center_y).abs();
///         for x in oval.left()..=oval.right() {
///             let fx = x as f64;
///             let adjusted_position_x = (fx - flocal_center_x).abs();
///             if adjusted_position_x <= fmin_bounds
///                 && adjusted_position_y <= fmin_bounds
///                 && ((fmin_bounds - adjusted_position_x.abs() >= 1.0)
///                     || (fmin_bounds - adjusted_position_y.abs() >= 1.0))
///             {
///                 let start_position = Position::new(
///                     adjusted_position_x.round() as Coord,
///                     adjusted_position_y.round() as Coord,
///                 );
///                 if adjusted_position_x >= (fmin_bounds - 1.0)
///                     && adjusted_position_x <= fmin_bounds
///                     && adjusted_position_y >= (fmin_bounds - 1.0)
///                     && adjusted_position_y <= fmin_bounds {
///                     queue.push_back(start_position);
///                 }
///                 visited.insert(start_position);
///             }
///         }
///     }
///     
///     // queue.push_back(start_position);
///     while !queue.is_empty() {
///         let current_position = queue.pop_front().unwrap();
///         if visited.contains(&current_position) {
///             continue;
///         }
///         visited.insert(current_position);
///
///         for y in -1..=1 {
///             for x in -1..=1 {
///                 let offset = Position::new(x, y);
///                 let test_position = current_position + offset;
///                 if !oval.intersects_local_position(test_position) {
///                     draw_oval(oval, &visited);
///                     panic!("Oval perimeter is not closed!");
///                 }
///                 match oval.contains_local_position(test_position) {
///                     // We have escaped the oval. The perimeter is not closed!
///                     Containment::Disjoint => {
///                         draw_oval(oval, &visited);
///                         panic!("Oval perimeter is not closed!");
///                     }
///                     Containment::Intersects => continue,
///                     Containment::Contains => {
///                         if !visited.contains(&test_position) {
///                             queue.push_back(test_position);
///                         }
///                     }
///                 }
///             }
///         }
///     }
/// }
///
/// (0..5_000).into_par_iter().for_each(|_i| {
///     let oval = Oval::new(
///         Position::new(0, 0),
///         SizeRange::new(Size::new(5, 5), Size::new(80, 80)).provide_size(),
///     );
///
///     test_oval(oval);
/// });
///
/// (5u32..80u32).into_par_iter().for_each(|i| {
///     let oval = Oval::new(
///         Position::new(0, 0),
///         Size::new(i, i),
///     );
///
///     test_oval(oval);
/// });
/// ```
#[derive(Copy, Clone, Debug, Display)]
pub struct Oval {
    area: Area,
}

impl Oval {
    /// Creates a new `Oval` given an [`Area`](struct.Position.html)
    ///
    /// The [`Position`](struct.Position.html) of the [`area`](struct.Area.html) is the top-left corner of the rectangle surrounding the oval, and the [`Size`](struct.Size.html) of the `Area` determines the bottom-right corner of the rectangle surrounding the oval.
    pub fn new(position: Position, size: Size) -> Self {
        Self {
            area: Area::new(position, size),
        }
    }
}

impl ContainsLocalPosition for Oval {
    fn contains_local_position(&self, position: Position) -> Containment {
        if !self.intersects_local_position(position) {
            return Containment::Disjoint;
        }

        let fwidth = ((self.area.width() as f64 / 2.0) - 0.5).max(0.0);
        let fheight = ((self.area.height() as f64 / 2.0) - 0.5).max(0.0);
        let fmin_bounds = (fwidth.min(fheight) - 1.0).max(0.0);
        let fmin_bounds = (fmin_bounds * fmin_bounds) / 2.0;
        let fmin_bounds = if fmin_bounds > 0.0 {
            fmin_bounds.sqrt()
        } else {
            fmin_bounds
        };
        let flocal_center_x = fwidth;
        let flocal_center_y = fheight;
        let adjusted_position_x = position.x() as f64 - flocal_center_x;
        let adjusted_position_y = position.y() as f64 - flocal_center_y;

        if adjusted_position_x.abs() <= fmin_bounds
            && adjusted_position_y.abs() <= fmin_bounds
            && ((fmin_bounds - adjusted_position_x.abs() >= 1.0)
                || (fmin_bounds - adjusted_position_y.abs() >= 1.0))
        {
            Containment::Contains
        } else if !(self.intersects_local_position(position + Position::NORTH)
            && self.intersects_local_position(position + Position::NORTH + Position::EAST)
            && self.intersects_local_position(position + Position::EAST)
            && self.intersects_local_position(position + Position::SOUTH + Position::EAST)
            && self.intersects_local_position(position + Position::SOUTH)
            && self.intersects_local_position(position + Position::SOUTH + Position::WEST)
            && self.intersects_local_position(position + Position::WEST)
            && self.intersects_local_position(position + Position::NORTH + Position::WEST))
        {
            Containment::Intersects
        } else {
            Containment::Contains
        }
    }
}

impl ContainsPosition for Oval {}

impl HasArea for Oval {
    fn area(&self) -> &Area {
        &self.area
    }

    fn area_mut(&mut self) -> &mut Area {
        &mut self.area
    }
}

impl HasHeight for Oval {
    fn height(&self) -> Length {
        self.size().height()
    }

    fn height_mut(&mut self) -> &mut Length {
        self.size_mut().height_mut()
    }
}

impl HasPosition for Oval {
    fn position(&self) -> &Position {
        self.area.position()
    }

    fn position_mut(&mut self) -> &mut Position {
        self.area.position_mut()
    }
}

impl HasSize for Oval {
    fn size(&self) -> &Size {
        self.area.size()
    }

    fn size_mut(&mut self) -> &mut Size {
        self.area.size_mut()
    }
}

impl HasWidth for Oval {
    fn width(&self) -> Length {
        self.size().width()
    }

    fn width_mut(&mut self) -> &mut Length {
        self.size_mut().width_mut()
    }
}

impl IntersectsLocalPosition for Oval {
    fn intersects_local_position(&self, position: Position) -> bool {
        let fwidth = ((self.area.width() as f64 / 2.0) - 0.5).max(0.0);
        let fheight = ((self.area.height() as f64 / 2.0) - 0.5).max(0.0);
        let ratio: f64 = fwidth / fheight;
        let flocal_center_x = fwidth;
        let flocal_center_y = fheight;
        let adjusted_position_x = position.x() as f64 - flocal_center_x;
        let adjusted_position_y = position.y() as f64 - flocal_center_y;

        let circular_position_x = adjusted_position_x / ratio;
        let circular_position_y = adjusted_position_y;

        let radius_sqr = fheight * fheight;
        let dist_sqr = (circular_position_x * circular_position_x)
            + (circular_position_y * circular_position_y);
        radius_sqr >= dist_sqr
    }
}

impl IntersectsPosition for Oval {}

impl Placed for Oval {}

impl PlacedObject for Oval {}

impl ProvidesArea for Oval {
    fn provide_area(&self) -> Area {
        self.area.provide_area()
    }
}

impl ProvidesPlacedShape for Oval {
    fn provide_placed_shape(&self) -> Box<dyn PlacedShape> {
        Box::new(*self)
    }
}

impl ProvidesSize for Oval {
    fn provide_size(&self) -> Size {
        self.area.provide_size()
    }
}

impl Shape for Oval {
    fn box_shape_clone(&self) -> Box<dyn Shape> {
        Box::new(*self)
    }
}
