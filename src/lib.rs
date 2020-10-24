#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! Defines various geometry enums, structs, traits, and types for the `dungen_minion` crate. As such, dungen_minion_geometry is not suitable as a general geometry crate.
//!
//! As the purpose of this crate is to provide data types for `dungen_minion` and `dungen_minion`'s other dependent crates to consume, the data types are defined here, while their general usages are defined in those other crates.

// External includes.
#[macro_use]
extern crate derive_more;

// Standard includes.

// Internal includes.
mod area;
mod area_range;
mod cardinal_direction;
mod cardinal_rotation;
mod containment;
mod contains_local_position;
mod contains_position;
mod count_range;
mod defines;
mod has_area;
mod has_bottom;
mod has_height;
mod has_left;
mod has_position;
mod has_right;
mod has_size;
mod has_top;
mod has_width;
mod intersects_local_position;
mod intersects_placed_shape;
mod intersects_position;
mod intersects_shape;
mod invert_placed_shape;
mod is_area;
mod is_position;
mod is_size;
mod oval;
mod placed;
mod placed_object;
mod placed_shape;
#[doc(inline)]
mod position;
mod position_range;
mod provides_area;
mod provides_count;
mod provides_placed_shape;
mod provides_position;
mod provides_shape;
mod provides_size;
mod shape;
mod size;
mod size_range;

pub use area::Area;
pub use area_range::AreaRange;
pub use cardinal_direction::CardinalDirection;
pub use cardinal_rotation::CardinalRotation;
pub use containment::Containment;
pub use contains_local_position::ContainsLocalPosition;
pub use contains_position::ContainsPosition;
pub use count_range::CountRange;
pub use defines::{Coord, Count, Length};
pub use has_area::HasArea;
pub use has_bottom::HasBottom;
pub use has_height::HasHeight;
pub use has_left::HasLeft;
pub use has_position::HasPosition;
pub use has_right::HasRight;
pub use has_size::HasSize;
pub use has_top::HasTop;
pub use has_width::HasWidth;
pub use intersects_local_position::IntersectsLocalPosition;
pub use intersects_placed_shape::IntersectsPlacedShape;
pub use intersects_position::IntersectsPosition;
pub use intersects_shape::IntersectsShape;
pub use invert_placed_shape::InvertPlacedShape;
pub use is_area::IsArea;
pub use is_position::IsPosition;
pub use is_size::IsSize;
pub use oval::Oval;
pub use placed::Placed;
pub use placed_object::PlacedObject;
pub use placed_shape::PlacedShape;
pub use position::Position;
pub use position_range::PositionRange;
pub use provides_area::ProvidesArea;
pub use provides_count::ProvidesCount;
pub use provides_placed_shape::ProvidesPlacedShape;
pub use provides_position::ProvidesPosition;
pub use provides_shape::ProvidesShape;
pub use provides_size::ProvidesSize;
pub use shape::Shape;
pub use size::Size;
pub use size_range::SizeRange;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
