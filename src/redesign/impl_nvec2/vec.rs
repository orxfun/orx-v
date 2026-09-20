use super::super::{D1, D2, Dim, NVec};
use alloc::vec::Vec;

impl<'a, T, V> NVec<D2, &'a T> for &'a Vec<V>
where
    &'a V: NVec<D1, &'a T>,
    T: 'a,
{
    fn at(&self, [i, j]: <D2 as Dim>::Idx) -> &'a T {
        (&self[i]).at(j)
    }
}
