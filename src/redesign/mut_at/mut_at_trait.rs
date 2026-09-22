use super::super::Dim;

pub trait MutAt<D: Dim, T> {
    fn mut_at(&mut self, idx: D::Idx) -> &mut T;
}
