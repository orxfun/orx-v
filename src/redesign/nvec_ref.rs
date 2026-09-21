use super::{Dim, NVec};

pub trait NVecRef<D: Dim, T> {
    fn ref_at(&self, idx: D::Idx) -> &T;
}
