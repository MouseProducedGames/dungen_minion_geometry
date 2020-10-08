// External includes.

// Standard includes.

// Internal includes.
use super::LocalPosition;

pub trait HasLocalPosition {
    fn local(&self) -> &LocalPosition;
    fn local_mut(&mut self) -> &mut LocalPosition;
}
