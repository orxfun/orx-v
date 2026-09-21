use super::{Dim, NVec};

pub trait NVecRef<D: Dim, T> {
    fn at(&self, idx: D::Idx) -> &T;
}
