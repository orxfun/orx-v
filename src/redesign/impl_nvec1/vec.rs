use super::super::{D1, Dim, NVec};
use alloc::vec::Vec;

impl<T: Copy> NVec<D1, T> for Vec<T> {
    #[inline(always)]
    fn at(&self, idx: <D1 as Dim>::Idx) -> T {
        self[idx]
    }
}

impl<T: Copy> NVec<D1, T> for &Vec<T> {
    #[inline(always)]
    fn at(&self, idx: <D1 as Dim>::Idx) -> T {
        self[idx]
    }
}

impl<'a, T> NVec<D1, &'a T> for &'a Vec<T> {
    #[inline(always)]
    fn at(&self, idx: <D1 as Dim>::Idx) -> &'a T {
        &self[idx]
    }
}
