use super::{Dim, NVec};

pub trait NVecRef<D: Dim, T> {
    fn at_ref(&self, idx: D::Idx) -> &T;

    // child

    type Child: NVecRef<D::PrevDim, T>;

    fn child(&self, child_idx: D::ChildIdx) -> &Self::Child;
}
