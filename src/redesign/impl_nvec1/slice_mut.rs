use super::super::{D1, Dim, NVecMut};

impl<'a, T> NVecMut<D1, T> for &'a mut [T] {
    #[inline(always)]
    fn at(&self, idx: <D1 as Dim>::Idx) -> &T {
        &self[idx]
    }

    #[inline(always)]
    fn at_mut(&mut self, idx: <D1 as Dim>::Idx) -> &mut T {
        &mut self[idx]
    }
}
