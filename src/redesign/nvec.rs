use super::Dim;

pub trait NVec<D: Dim, T> {
    fn at(&self, idx: D::Idx) -> T;
}
