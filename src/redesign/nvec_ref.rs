use super::{Dim, NVec};

pub trait NVecRef<D: Dim, T> {
    fn at_ref(&self, idx: D::Idx) -> &T;
}

impl<'a, D: Dim, T, V: NVecRef<D, T>> NVec<D, &'a T> for &'a V {
    #[inline(always)]
    fn at(&self, idx: <D as Dim>::Idx) -> &'a T {
        <V as NVecRef<D, T>>::at_ref(self, idx)
    }
}
