use super::super::{D1, Dim, NVec, NVecMut};
use alloc::vec::Vec;

impl<'a, T> NVec<D1, &'a T> for &'a Vec<T> {
    #[inline(always)]
    fn at(&self, idx: <D1 as Dim>::Idx) -> &'a T {
        &self[idx]
    }
}

impl<T> NVecMut<D1, T> for Vec<T> {
    #[inline(always)]
    fn at_mut(&mut self, idx: <D1 as Dim>::Idx) -> &mut T {
        &mut self[idx]
    }
}
