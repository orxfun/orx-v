use super::{Dim, NVec};

pub trait NVecMut<D: Dim, T> {
    fn at(&self, idx: D::Idx) -> &T;

    fn at_mut(&mut self, idx: D::Idx) -> &mut T;
}

impl<'a, D: Dim, T, V: NVecMut<D, T>> NVec<D, &'a T> for &'a V {
    #[inline(always)]
    fn at(&self, idx: <D as Dim>::Idx) -> &'a T {
        <V as NVecMut<D, T>>::at(self, idx)
    }
}
