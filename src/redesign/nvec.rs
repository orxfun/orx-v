use super::Dim;

pub trait NVec<D: Dim, T> {
    fn at(&self, idx: impl Into<D::Idx>) -> T;
}
