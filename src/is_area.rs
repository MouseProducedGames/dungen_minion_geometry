// External includes.

// Standard includes.

// Internal includes.
use super::{HasArea, HasPosition, HasSize, IsSize, Length};

/// `IsArea` is defined as both [`HasPosition`](trait.HasPosition.html), [`HasSize`](trait.HasSize.html), and [`IsSize`](trait.IsSize.html).
pub trait IsArea: HasArea + HasPosition + HasSize + IsSize {}

impl<TIsArea> IsSize for TIsArea
where
    TIsArea: IsArea,
{
    fn height(&self) -> Length {
        self.size().height()
    }

    fn height_mut(&mut self) -> &mut Length {
        self.size_mut().height_mut()
    }

    fn width(&self) -> Length {
        self.size().width()
    }

    fn width_mut(&mut self) -> &mut Length {
        self.size_mut().width_mut()
    }
}
