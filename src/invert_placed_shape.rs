// External includes.

// Standard includes.

// Internal includes.
use super::{
    Area, Containment, ContainsLocalPosition, ContainsPosition, HasArea, HasPosition, HasSize,
    IntersectsLocalPosition, IntersectsPosition, IsArea, IsSize, Placed, PlacedObject, PlacedShape,
    Position, ProvidesArea, ProvidesSize, Shape, Size,
};

/// Inverts the [`Containment`](enum.Containment.html) and intersection of the contained [`PlacedShape`](trait.PlacedShape.html).
///
/// ```
/// # use dungen_minion_geometry::*;
/// let area = Area::new(Position::new(0, 0), Size::new(3, 3));
/// let inverted_area = InvertPlacedShape::new(area);
///
/// // Positions outside of the inverted area register as contained in the shape.
/// assert!(inverted_area.contains_position(Position::new(-1, -1)) == Containment::Contains);
/// // Positions on the edge of the inverted area register as intersecting the shape.
/// assert!(inverted_area.contains_position(Position::new(0, 0)) == Containment::Intersects);
/// // Positions inside of of the inverted area register as disjoint from the shape.
/// assert!(inverted_area.contains_position(Position::new(1, 1)) == Containment::Disjoint);
///
/// // Positions outside of the inverted area register as intersecting it.
/// assert!(inverted_area.intersects_position(Position::new(-1, -1)) == true);
/// // Positions on the edge of the inverted area register as intersecting it.
/// assert!(inverted_area.intersects_position(Position::new(0, 0)) == true);
/// // Positions inside of the inverted area do not register as intersecting it.
/// assert!(inverted_area.intersects_position(Position::new(1, 1)) == false);
/// ```
pub struct InvertPlacedShape<TPlacedShape>
where
    TPlacedShape: PlacedShape + ProvidesArea + ProvidesSize + Sized,
{
    inner: TPlacedShape,
}

impl<TPlacedShape> InvertPlacedShape<TPlacedShape>
where
    TPlacedShape: PlacedShape + ProvidesArea + ProvidesSize + Sized,
{
    /// Creates a new inverted `PlacedShape` out of the inner `PlacedShape`.
    pub fn new(inner: TPlacedShape) -> Self {
        Self { inner }
    }
}

impl<TPlacedShape> ContainsLocalPosition for InvertPlacedShape<TPlacedShape>
where
    TPlacedShape: PlacedShape + ProvidesArea + ProvidesSize,
{
    fn contains_local_position(&self, position: Position) -> Containment {
        match self.inner.contains_local_position(position) {
            Containment::Disjoint => Containment::Contains,
            Containment::Intersects => Containment::Intersects,
            Containment::Contains => Containment::Disjoint,
        }
    }
}

impl<TPlacedShape> ContainsPosition for InvertPlacedShape<TPlacedShape> where
    TPlacedShape: PlacedShape + ProvidesArea + ProvidesSize
{
}

impl<TPlacedShape> HasArea for InvertPlacedShape<TPlacedShape>
where
    TPlacedShape: PlacedShape + ProvidesArea + ProvidesSize,
{
    fn area(&self) -> &Area {
        self.inner.area()
    }

    fn area_mut(&mut self) -> &mut Area {
        self.inner.area_mut()
    }
}

impl<TPlacedShape> HasPosition for InvertPlacedShape<TPlacedShape>
where
    TPlacedShape: PlacedShape + ProvidesArea + ProvidesSize,
{
    fn position(&self) -> &Position {
        self.inner.position()
    }

    fn position_mut(&mut self) -> &mut Position {
        self.inner.position_mut()
    }
}

impl<TPlacedShape> HasSize for InvertPlacedShape<TPlacedShape>
where
    TPlacedShape: PlacedShape + ProvidesArea + ProvidesSize,
{
    fn size(&self) -> &Size {
        self.inner.size()
    }

    fn size_mut(&mut self) -> &mut Size {
        self.inner.size_mut()
    }
}

impl<TPlacedShape> IntersectsLocalPosition for InvertPlacedShape<TPlacedShape>
where
    TPlacedShape: PlacedShape + ProvidesArea + ProvidesSize,
{
    fn intersects_local_position(&self, position: Position) -> bool {
        self.contains_local_position(position) != Containment::Disjoint
    }
}

impl<TPlacedShape> IntersectsPosition for InvertPlacedShape<TPlacedShape> where
    TPlacedShape: PlacedShape + ProvidesArea + ProvidesSize
{
}

impl<TPlacedShape> IsArea for InvertPlacedShape<TPlacedShape> where
    TPlacedShape: PlacedShape + ProvidesArea + ProvidesSize
{
}

impl<TPlacedShape> IsSize for InvertPlacedShape<TPlacedShape> where
    TPlacedShape: PlacedShape + ProvidesArea + ProvidesSize
{
}

impl<TPlacedShape> Placed for InvertPlacedShape<TPlacedShape> where
    TPlacedShape: PlacedShape + ProvidesArea + ProvidesSize
{
}

impl<TPlacedShape> PlacedObject for InvertPlacedShape<TPlacedShape> where
    TPlacedShape: PlacedShape + ProvidesArea + ProvidesSize
{
}

impl<TPlacedShape> ProvidesArea for InvertPlacedShape<TPlacedShape>
where
    TPlacedShape: PlacedShape + ProvidesArea + ProvidesSize,
{
    fn provide_area(&self) -> Area {
        self.inner.provide_area()
    }
}

impl<TPlacedShape> ProvidesSize for InvertPlacedShape<TPlacedShape>
where
    TPlacedShape: PlacedShape + ProvidesArea + ProvidesSize,
{
    fn provide_size(&self) -> Size {
        self.inner.provide_size()
    }
}

impl<TPlacedShape> Shape for InvertPlacedShape<TPlacedShape> where
    TPlacedShape: PlacedShape + ProvidesArea + ProvidesSize
{
}
