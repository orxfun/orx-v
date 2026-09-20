use super::super::{D1, D2, Dim, NVec, NVecMut, NVecRef};
use alloc::vec::Vec;

impl<T, V: NVecRef<D1, T>> NVecRef<D2, T> for Vec<V> {
    fn at_ref(&self, [i, j]: <D2 as Dim>::Idx) -> &T {
        self[i].at_ref(j)
    }
}

impl<T, V: NVecMut<D1, T>> NVecMut<D2, T> for Vec<V> {
    fn at_mut(&mut self, [i, j]: <D2 as Dim>::Idx) -> &mut T {
        self[i].at_mut(j)
    }
}

impl<'a, T, V> NVec<D2, &'a T> for &'a Vec<V>
where
    &'a V: NVec<D1, &'a T>,
{
    fn at(&self, [i, j]: <D2 as Dim>::Idx) -> &'a T {
        let child: &'a V = &self[i];
        child.at(j)
    }
}
