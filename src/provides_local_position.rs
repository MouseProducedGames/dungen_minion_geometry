// External includes.

// Standard includes.

// Internal includes.
use super::LocalPosition;

pub trait ProvidesLocalPosition {
    fn provide_local(&self) -> LocalPosition;
}
