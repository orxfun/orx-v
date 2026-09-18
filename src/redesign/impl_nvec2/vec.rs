use super::super::{D1, D2, Dim, NVec};
use alloc::vec::Vec;

impl<T: Copy, V> NVec<D2, T> for Vec<V>
where
    V: NVec<D1, T>,
{
    fn at(&self, [i, j]: <D2 as Dim>::Idx) -> T {
        self[i].at(j)
    }
}

impl<'a, T, V> NVec<D2, &'a T> for &'a Vec<V>
where
    V: NVec<D1, &'a T>,
    T: 'a,
{
    fn at(&self, [i, j]: <D2 as Dim>::Idx) -> &'a T {
        self[i].at(j)
    }
}
