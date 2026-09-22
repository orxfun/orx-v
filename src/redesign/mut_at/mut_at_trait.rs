use super::super::Dim;

pub trait MutAt<D: Dim, T> {
    fn at(&self, idx: D::Idx) -> &T;

    fn mut_at(&mut self, idx: D::Idx) -> &mut T;
}
