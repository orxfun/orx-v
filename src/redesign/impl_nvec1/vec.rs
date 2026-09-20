use super::super::{D1, Dim, NVecMut, NVecRef};
use alloc::vec::Vec;

impl<T> NVecRef<D1, T> for Vec<T> {
    #[inline(always)]
    fn at_ref(&self, idx: <D1 as Dim>::Idx) -> &T {
        &self[idx]
    }
}

impl<T> NVecMut<D1, T> for Vec<T> {
    #[inline(always)]
    fn at_mut(&mut self, idx: <D1 as Dim>::Idx) -> &mut T {
        &mut self[idx]
    }
}
