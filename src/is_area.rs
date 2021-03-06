// External includes.

// Standard includes.

// Internal includes.
use super::{HasArea, HasHeight, HasWidth, IsSize, Length, ProvidesArea};

/// `IsArea` is defined as both [`HasPosition`](trait.HasPosition.html), [`HasSize`](trait.HasSize.html), and [`IsSize`](trait.IsSize.html).
pub trait IsArea: HasArea + IsSize + ProvidesArea {}

impl<TIsArea> HasHeight for TIsArea
where
    TIsArea: IsArea,
{
    fn height(&self) -> Length {
        self.size().height()
    }

    fn height_mut(&mut self) -> &mut Length {
        self.size_mut().height_mut()
    }
}

impl<TIsArea> HasWidth for TIsArea
where
    TIsArea: IsArea,
{
    fn width(&self) -> Length {
        self.size().width()
    }

    fn width_mut(&mut self) -> &mut Length {
        self.size_mut().width_mut()
    }
}
