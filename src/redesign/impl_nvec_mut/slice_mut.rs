use super::super::{D1, Dim, NVecMut};

impl<'a, T> NVecMut<D1, &'a T> for &'a mut [T] {
    type ItemMut<'b>
        = &'a mut T
    where
        Self: 'b;

    fn at_mut(&mut self, idx: impl Into<<D1 as Dim>::Idx>) -> Self::ItemMut<'_> {
        &mut self[idx.into()]
    }
}
