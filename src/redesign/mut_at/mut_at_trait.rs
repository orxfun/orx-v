use super::super::Dim;

pub trait MutAt<D: Dim, T> {
    fn at(&self, idx: D::Idx) -> &T;

    fn try_at(&self, idx: D::Idx) -> Option<&T>;

    fn mut_at(&mut self, idx: D::Idx) -> &mut T;

    fn try_mut_at(&mut self, idx: D::Idx) -> Option<&mut T>;
}
