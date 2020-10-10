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
mod defines;
mod has_area;
mod has_position;
mod has_shape_position;
mod has_size;
mod intersects;
mod intersects_local;
mod intersects_pos;
mod intersects_shape_pos;
mod is_area;
mod is_position;
mod is_shape_position;
mod is_size;
mod ordinal_direction;
mod ordinal_rotation;
mod placed;
mod placed_object;
mod placed_shape;
#[doc(inline)]
mod position;
mod position_range;
mod provides_area;
mod provides_position;
mod provides_shape_position;
mod provides_size;
mod shape;
mod shape_position;
mod shape_position_range;
mod size;
mod size_range;

pub use area::Area;
pub use area_range::AreaRange;
pub use defines::{Coord, Length};
pub use has_area::HasArea;
pub use has_position::HasPosition;
pub use has_shape_position::HasShapePosition;
pub use has_size::HasSize;
pub use intersects::Intersects;
pub use intersects_local::IntersectsLocal;
pub use intersects_pos::IntersectsPos;
pub use intersects_shape_pos::IntersectsShapePosition;
pub use is_area::IsArea;
pub use is_position::IsPosition;
pub use is_shape_position::IsShapePosition;
pub use is_size::IsSize;
pub use ordinal_direction::OrdinalDirection;
pub use ordinal_rotation::OrdinalRotation;
pub use placed::Placed;
pub use placed_object::PlacedObject;
pub use placed_shape::PlacedShape;
pub use position::Position;
pub use position_range::PositionRange;
pub use provides_area::ProvidesArea;
pub use provides_position::ProvidesPosition;
pub use provides_shape_position::ProvidesShapePosition;
pub use provides_size::ProvidesSize;
pub use shape::Shape;
pub use shape_position::ShapePosition;
pub use shape_position_range::ShapePositionRange;
pub use size::Size;
pub use size_range::SizeRange;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
