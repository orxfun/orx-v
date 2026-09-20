use super::super::{D1, Dim, NVecMut, NVecRef};

impl<'a, T> NVecRef<D1, T> for &'a mut [T] {
    #[inline(always)]
    fn at_ref(&self, idx: <D1 as Dim>::Idx) -> &T {
        &self[idx]
    }
}

impl<'a, T> NVecMut<D1, T> for &'a mut [T] {
    #[inline(always)]
    fn at_mut(&mut self, idx: <D1 as Dim>::Idx) -> &mut T {
        &mut self[idx]
    }
}
