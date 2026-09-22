use super::super::Dim;

pub trait AtMut<D: Dim, T> {
    fn at(&self, idx: D::Idx) -> &T;

    fn try_at(&self, idx: D::Idx) -> Option<&T>;

    fn at_mut(&mut self, idx: D::Idx) -> &mut T;

    fn try_at_mut(&mut self, idx: D::Idx) -> Option<&mut T>;
}
