// External includes.

// Standard includes.

// Internal includes.
use super::{
    Area, Containment, ContainsLocalPosition, ContainsPosition, Coord, HasArea, HasPosition,
    HasSize, IntersectsLocalPosition, IntersectsPosition, IsPosition, IsSize, Length, Placed,
    PlacedObject, Position, Shape, Size,
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
/// fn draw_oval(oval: Oval) {
///     for y in oval.top()..=oval.bottom() {
///         for x in oval.left()..=oval.right() {
///             let position = Position::new(x, y);
///             let draw_ch = match oval.contains_local_position(position) {
///                 Containment::Disjoint => ' ',
///                 Containment::Intersects => '#',
///                 Containment::Contains => '.',
///             };
///             print!("{}", draw_ch);
///         }
///         println!();
///     }
/// }
///
/// [0..5_000].par_iter().for_each(|_i| {
///     let oval = Oval::new(Area::new(
///         Position::new(0, 0),
///         SizeRange::new(Size::new(10, 10), Size::new(80, 80)).provide_size(),
///     ));
///
///     // Perform a flood-fill to ensure the oval is bounded.
///     let mut queue = VecDeque::new();
///     let mut visited = HashSet::new();
///     // Given the minimum size of the test oval, we get a point in the middle which should
///     // be guaranteed empty.
///     let start_position = Position::new(
///         (oval.left() / 2) + (oval.right() / 2),
///         (oval.top() / 2) + (oval.bottom() / 2),
///     );
///     // We test it anyway, just in case.
///     if oval.contains_local_position(start_position) != Containment::Contains {
///         draw_oval(oval);
///         // The assert is not simplified, for tracking purposes.
///         assert!(oval.contains_local_position(start_position) == Containment::Contains);
///         // And if the assert doesn't fire, we panic.
///         panic!(
///             "Assert that should have fired, didn't! {}",
///             oval.contains_local_position(start_position)
///         );
///     }
///
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
///     for y in oval.top()..=oval.bottom() {
///         let fy = y as f64;
///         let adjusted_position_y = (fy - flocal_center_y).abs();
///         for x in oval.left()..=oval.right() {
///             let fx = x as f64;
///             let adjusted_position_x = (fx - flocal_center_x).abs();
///             if adjusted_position_x <= fmin_bounds
///                 && adjusted_position_y <= fmin_bounds
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
///                     continue;
///                 }
///                 match oval.contains_local_position(test_position) {
///                     // We have escaped the oval. The perimeter is not closed!
///                     Containment::Disjoint => {
///                         draw_oval(oval);
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
///     // draw_oval(oval);
/// });
/// // panic!("Test.");
/// ```
#[derive(Copy, Clone, Debug, Display)]
pub struct Oval {
    area: Area,
}

impl Oval {
    /// Creates a new `Oval` given an [`Area`](struct.Position.html)
    ///
    /// The [`Position`](struct.Position.html) of the [`area`](struct.Area.html) is the top-left corner of the rectangle surrounding the oval, and the [`Size`](struct.Size.html) of the `Area` determines the bottom-right corner of the rectangle surrounding the oval.
    pub fn new(area: Area) -> Self {
        Self { area }
    }

    /// The left-most coordinate of the oval.
    ///
    /// Horizontal coordinates increase towards the east.
    pub fn left(&self) -> Coord {
        self.position().x()
    }

    /// A mutable reference to the left-most coordinate of the oval.
    ///
    /// Horizontal coordinates increase towards the east.
    pub fn left_mut(&mut self) -> &mut Coord {
        self.position_mut().x_mut()
    }

    /// The top-most coordinate of the oval.
    ///
    /// Vertical coordinates increase towards the south.
    pub fn top(&self) -> Coord {
        self.position().y()
    }

    /// A mutable reference to the top-most coordinate of the oval.
    ///
    /// Vertical coordinates increase towards the south.
    pub fn top_mut(&mut self) -> &mut Coord {
        self.position_mut().y_mut()
    }

    /// The right-most coordinate of the oval.
    ///
    /// A geometic tile area with a width of 1, has the same right tile as its left tile.
    pub fn right(&self) -> Coord {
        self.position().x() + (self.size().width() as Coord - 1).max(0)
    }

    /// Sets the right-most coordinate of the oval.
    ///
    /// Cannot set the right-most coordinate to less than the x-coordinate.
    pub fn right_set(&mut self, value: Coord) {
        let width_coord = (value - self.position().x()) + 1;
        let width = width_coord.max(0) as Length;
        *self.size_mut().width_mut() = width;
    }

    /// The bottom-most coordinate of the oval.
    ///
    /// A geometic tile area with a height of 1, has the same bottom tile as its top tile.
    pub fn bottom(&self) -> Coord {
        self.position().y() + (self.size().height() as Coord - 1).max(1)
    }

    /// Sets the bottom-most coordinate of the oval.
    ///
    /// Cannot set the bottom-most coordinate to less than the y-coordinate.
    pub fn bottom_set(&mut self, value: Coord) {
        let height_coord = (value - self.position().y()) + 1;
        let height = height_coord.max(0) as Length;
        *self.size_mut().height_mut() = height;
    }
}

impl ContainsLocalPosition for Oval {
    fn contains_local_position(&self, position: Position) -> Containment {
        /* let fwidth = ((self.area.width() as f64 / 2.0) - 0.5).max(0.0);
        let fheight = ((self.area.height() as f64 / 2.0) - 0.5).max(0.0);
        let ratio: f64 = fwidth / fheight;
        let flocal_center_x = self.left() as f64 * 0.5 + self.right() as f64 * 0.5;
        let flocal_center_y = self.top() as f64 * 0.5 + self.bottom() as f64 * 0.5;
        let adjusted_position_x = position.x() as f64 - flocal_center_x;
        let adjusted_position_y = position.y() as f64 - flocal_center_y;

        let test_value = ((adjusted_position_x * adjusted_position_x) / (fwidth * fwidth))
            + (adjusted_position_y * adjusted_position_y) / (fheight * fheight);

        if test_value > 1.0 {
            Containment::Disjoint
        } else if test_value == 1.0 {
            Containment::Intersects
        } else {
            Containment::Contains
        } */

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

        if adjusted_position_x.abs() <= fmin_bounds && adjusted_position_y.abs() <= fmin_bounds {
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

        /* // let fwidth = self.area.width() as f64 / 2.0;
        // let fheight = self.area.height() as f64 / 2.0;
        let fwidth = ((self.area.width() as f64 / 2.0) - 0.5).max(0.0);
        let fheight = ((self.area.height() as f64 / 2.0) - 0.5).max(0.0);
        let ratio: f64 = fwidth / fheight;
        let flocal_center_x = self.left() as f64 * 0.5 + self.right() as f64 * 0.5;
        let flocal_center_y = self.top() as f64 * 0.5 + self.bottom() as f64 * 0.5;
        let adjusted_position_x = position.x() as f64 - flocal_center_x;
        let adjusted_position_y = position.y() as f64 - flocal_center_y;

        // print!("( {}, {} ) = {}", adjusted_position_x, adjusted_position_y, intersection_check);

        let circular_position_x = adjusted_position_x / ratio;
        let circular_position_y = adjusted_position_y;

        const SQRT2: f64 = 1.4142135623730950488016887242097f64;
        let intersection_check =
            if (circular_position_x.abs() - circular_position_y.abs()).abs() >= f64::EPSILON {
                let value_x = (adjusted_position_x / (fwidth - 0.5).max(0.0)).abs();
                let value_y = (adjusted_position_y / (fheight - 0.5).max(0.0)).abs();
                let mul_x = (1.0 * (1.0 - value_y)) + (SQRT2 * value_y);
                let mul_y = (1.0 * (1.0 - value_x)) + (SQRT2 * value_x);
                // let mul_x2 = mul_x / mul_y;
                // let mul_y2 = mul_y / mul_x;
                // let mul = mul_x2 * mul_y2;
                /* let mul = ((mul_x * mul_y).abs() + (mul_y * mul_x).abs())
                    / ((mul_x * mul_x) + (mul_y * mul_y));
                let mul_x2 = ((mul_x * mul_x) + (mul_x * mul_x))
                    / ((mul_x * mul_y).abs() + (mul_y * mul_x).abs());
                let mul_y2 = ((mul_y * mul_y) + (mul_y * mul_y))
                    / ((mul_x * mul_y).abs() + (mul_y * mul_x).abs()); */
                // let value_x2 = value_x * ((mul_x2 * (1.0 - mul)) + (mul_x * mul));
                // let value_y2 = value_y * ((mul_y2 * (1.0 - mul)) + (mul_y * mul));
                // let mul_x2 = ((mul_x * mul_x) + (mul_x * mul_x)) / ((2.0) + (2.0));
                // let mul_y2 = ((mul_y * mul_y) + (mul_y * mul_y)) / ((2.0) + (2.0));
                /* let mul_x2 = ((mul_x * mul_y)) / ((2.0));
                let mul_y2 = ((mul_x * mul_y)) / ((2.0)); */
                let mul = ((mul_x * mul_y).abs() + (mul_y * mul_x).abs()) / (4.0);
                let value_x2 = value_x * (mul_x * mul);
                let value_y2 = value_y * (mul_y * mul);
                (value_x2 / ratio) + value_y2
            // let interpolation = interpolation_x.min(interpolation_y);
            // ((1.0 * (1.0 - interpolation)) + ((1.0 / circle_ratio) * interpolation))
            /* let interpolation = circular_position_x.abs()
                / (circular_position_x.abs() + circular_position_y.abs());
            (1.0 * interpolation) * (value * (1.0 - interpolation)) */
            // + ((0.0 * (1.0 - interpolation_y)) + (1.0 * interpolation_x))
            } else {
                1.0
            };
        let inverse_intersection_check = 1.0 / intersection_check;

        let length_sqr = (adjusted_position_x * adjusted_position_x)
            + (adjusted_position_y * adjusted_position_y);
        let length = length_sqr.sqrt();
        let size_length_sqr = (fheight * fheight) + (fheight * fheight);
        let size_length = size_length_sqr.sqrt();
        let length_position_x = (adjusted_position_x / length) * size_length * ratio;
        let length_position_y = (adjusted_position_y / length) * size_length;
        let diff_x = length_position_x.abs() - adjusted_position_x.abs();
        let diff_y = length_position_y.abs() - adjusted_position_y.abs();

        const SQRT8: f64 = 2.8284271247461900976033774484194f64;
        let intersection = intersection_check.max(inverse_intersection_check).max(SQRT8);
        let radius = fheight;
        let radius2 = (radius - intersection).max(0.0);
        let dist = ((circular_position_x * circular_position_x)
            + (circular_position_y * circular_position_y))
            .sqrt();
        let diff = radius - dist;
        if radius < dist {
            Containment::Disjoint
        } else if (radius2 <= dist
            || (diff_x >= 0.0 && diff_x <= intersection && diff_y >= 0.0 && diff_y <= intersection))
            && !(self.intersects_local_position(position + Position::NORTH)
                && self.intersects_local_position(position + Position::NORTH + Position::EAST)
                && self.intersects_local_position(position + Position::EAST)
                && self.intersects_local_position(position + Position::SOUTH + Position::EAST)
                && self.intersects_local_position(position + Position::SOUTH)
                && self.intersects_local_position(position + Position::SOUTH + Position::WEST)
                && self.intersects_local_position(position + Position::WEST)
                && self.intersects_local_position(position + Position::NORTH + Position::WEST))
        // (diff_x >= 0.0 && diff_x <= 1.0) && (diff_y >= 0.0 && diff_y <= 1.0)
        /* diff <= intersection_check * 0.5)
        // || (diff <= (intersection_check.max(inverse_intersection_check) * SQRT8).max(SQRT8)
        || (((adjusted_position_x.abs() - length_position_x.abs())
        ||)
            && !(self.intersects_local_position(position + Position::NORTH)
                && self.intersects_local_position(position + Position::NORTH + Position::EAST)
                && self.intersects_local_position(position + Position::EAST)
                && self.intersects_local_position(position + Position::SOUTH + Position::EAST)
                && self.intersects_local_position(position + Position::SOUTH)
                && self.intersects_local_position(position + Position::SOUTH + Position::WEST)
                && self.intersects_local_position(position + Position::WEST)
                && self.intersects_local_position(position + Position::NORTH + Position::WEST))) */
        {
            Containment::Intersects
        } else {
            Containment::Contains
        } */
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

impl IntersectsLocalPosition for Oval {
    fn intersects_local_position(&self, position: Position) -> bool {
        let fwidth = ((self.area.width() as f64 / 2.0) - 0.5).max(0.0);
        let fheight = ((self.area.height() as f64 / 2.0) - 0.5).max(0.0);
        let ratio: f64 = fwidth / fheight;
        let flocal_center_x = fwidth;
        let flocal_center_y = fheight;
        // let flocal_center_x = self.left() as f64 * 0.5 + self.right() as f64 * 0.5;
        // let flocal_center_y = self.top() as f64 * 0.5 + self.bottom() as f64 * 0.5;
        let adjusted_position_x = position.x() as f64 - flocal_center_x;
        let adjusted_position_y = position.y() as f64 - flocal_center_y;

        // print!("( {}, {} ) = {}", adjusted_position_x, adjusted_position_y, intersection_check);

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

impl Shape for Oval {}
