// External includes.

// Standard includes.

// Internal includes.
mod area;
mod defines;
mod has_area;
mod has_local_position;
mod has_position;
mod has_size;
mod intersects;
mod intersects_local;
mod intersects_local_pos;
mod intersects_pos;
mod is_area;
mod is_local_position;
mod is_position;
mod is_size;
mod local_position;
mod placed;
mod placed_object;
mod placed_shape;
mod position;
mod provides_area;
mod provides_local_position;
mod provides_position;
mod provides_size;
mod shape;
mod size;

pub use area::Area;
pub use defines::{Coord, Length};
pub use has_area::HasArea;
pub use has_local_position::HasLocalPosition;
pub use has_position::HasPosition;
pub use has_size::HasSize;
pub use intersects::Intersects;
pub use intersects_local::IntersectsLocal;
pub use intersects_local_pos::IntersectsLocalPos;
pub use intersects_pos::IntersectsPos;
pub use is_area::IsArea;
pub use is_local_position::IsLocalPosition;
pub use is_position::IsPosition;
pub use is_size::IsSize;
pub use local_position::LocalPosition;
pub use placed::Placed;
pub use placed_object::PlacedObject;
pub use placed_shape::PlacedShape;
pub use position::Position;
pub use provides_area::ProvidesArea;
pub use provides_local_position::ProvidesLocalPosition;
pub use provides_position::ProvidesPosition;
pub use provides_size::ProvidesSize;
pub use shape::Shape;
pub use size::Size;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
