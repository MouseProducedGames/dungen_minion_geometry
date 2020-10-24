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
#[derive(Clone)]
pub struct InvertPlacedShape<TPlacedShape>
where
    TPlacedShape: Clone + Clone + PlacedShape + ProvidesArea + ProvidesSize + Sized + Sized,
{
    inner: TPlacedShape,
}

impl<TPlacedShape: 'static> InvertPlacedShape<TPlacedShape>
where
    TPlacedShape: Clone + Clone + PlacedShape + ProvidesArea + ProvidesSize + Sized + Sized,
{
    /// Creates a new inverted `PlacedShape` out of the inner `PlacedShape`.
    pub fn new(inner: TPlacedShape) -> Self {
        Self { inner }
    }
}

impl<TPlacedShape: 'static> ContainsLocalPosition for InvertPlacedShape<TPlacedShape>
where
    TPlacedShape: Clone + PlacedShape + ProvidesArea + ProvidesSize + Sized,
{
    fn contains_local_position(&self, position: Position) -> Containment {
        match self.inner.contains_local_position(position) {
            Containment::Disjoint => Containment::Contains,
            Containment::Intersects => Containment::Intersects,
            Containment::Contains => Containment::Disjoint,
        }
    }
}

impl<TPlacedShape: 'static> ContainsPosition for InvertPlacedShape<TPlacedShape> where
    TPlacedShape: Clone + PlacedShape + ProvidesArea + ProvidesSize + Sized
{
}

impl<TPlacedShape: 'static> HasArea for InvertPlacedShape<TPlacedShape>
where
    TPlacedShape: Clone + PlacedShape + ProvidesArea + ProvidesSize + Sized,
{
    fn area(&self) -> &Area {
        self.inner.area()
    }

    fn area_mut(&mut self) -> &mut Area {
        self.inner.area_mut()
    }
}

impl<TPlacedShape: 'static> HasPosition for InvertPlacedShape<TPlacedShape>
where
    TPlacedShape: Clone + PlacedShape + ProvidesArea + ProvidesSize + Sized,
{
    fn position(&self) -> &Position {
        self.inner.position()
    }

    fn position_mut(&mut self) -> &mut Position {
        self.inner.position_mut()
    }
}

impl<TPlacedShape: 'static> HasSize for InvertPlacedShape<TPlacedShape>
where
    TPlacedShape: Clone + PlacedShape + ProvidesArea + ProvidesSize + Sized,
{
    fn size(&self) -> &Size {
        self.inner.size()
    }

    fn size_mut(&mut self) -> &mut Size {
        self.inner.size_mut()
    }
}

impl<TPlacedShape: 'static> IntersectsLocalPosition for InvertPlacedShape<TPlacedShape>
where
    TPlacedShape: Clone + PlacedShape + ProvidesArea + ProvidesSize + Sized,
{
    fn intersects_local_position(&self, position: Position) -> bool {
        self.contains_local_position(position) != Containment::Disjoint
    }
}

impl<TPlacedShape: 'static> IntersectsPosition for InvertPlacedShape<TPlacedShape> where
    TPlacedShape: Clone + PlacedShape + ProvidesArea + ProvidesSize + Sized
{
}

impl<TPlacedShape: 'static> IsArea for InvertPlacedShape<TPlacedShape> where
    TPlacedShape: Clone + PlacedShape + ProvidesArea + ProvidesSize + Sized
{
}

impl<TPlacedShape: 'static> IsSize for InvertPlacedShape<TPlacedShape> where
    TPlacedShape: Clone + PlacedShape + ProvidesArea + ProvidesSize + Sized
{
}

impl<TPlacedShape: 'static> Placed for InvertPlacedShape<TPlacedShape> where
    TPlacedShape: Clone + PlacedShape + ProvidesArea + ProvidesSize + Sized
{
}

impl<TPlacedShape: 'static> PlacedObject for InvertPlacedShape<TPlacedShape> where
    TPlacedShape: Clone + PlacedShape + ProvidesArea + ProvidesSize + Sized
{
}

impl<TPlacedShape: 'static> ProvidesArea for InvertPlacedShape<TPlacedShape>
where
    TPlacedShape: Clone + PlacedShape + ProvidesArea + ProvidesSize + Sized,
{
    fn provide_area(&self) -> Area {
        self.inner.provide_area()
    }
}

impl<TPlacedShape: 'static> ProvidesSize for InvertPlacedShape<TPlacedShape>
where
    TPlacedShape: Clone + PlacedShape + ProvidesArea + ProvidesSize + Sized,
{
    fn provide_size(&self) -> Size {
        self.inner.provide_size()
    }
}

impl<TPlacedShape: 'static> Shape for InvertPlacedShape<TPlacedShape>
where
    TPlacedShape: Clone + PlacedShape + ProvidesArea + ProvidesSize + Sized,
{
    fn box_shape_clone(&self) -> Box<dyn Shape> {
        Box::new((*self).clone())
    }
}
