use super::super::{D1, Dim, NVecMut};
use alloc::vec::Vec;

impl<T> NVecMut<D1, T> for Vec<T> {
    type ItemMut<'a>
        = &'a mut T
    where
        Self: 'a;

    fn at_mut(&mut self, idx: impl Into<<D1 as Dim>::Idx>) -> Self::ItemMut<'_> {
        &mut self[idx.into()]
    }
}
