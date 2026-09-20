use super::Dim;

pub trait NVecRef<D: Dim, T> {
    fn at_ref(&self, idx: D::Idx) -> &T;
}
