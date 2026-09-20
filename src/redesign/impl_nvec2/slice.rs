use super::super::{D1, D2, Dim, NVec};

impl<'a, T, V> NVec<D2, &'a T> for &'a [V]
where
    &'a V: NVec<D1, &'a T>,
{
    #[inline(always)]
    fn at(&self, [i, j]: <D2 as Dim>::Idx) -> &'a T {
        (&self[i]).at(j)
    }
}

impl<'a, T, V> NVec<D2, &'a T> for &&'a [V]
where
    &'a V: NVec<D1, &'a T>,
{
    #[inline(always)]
    fn at(&self, [i, j]: <D2 as Dim>::Idx) -> &'a T {
        (&self[i]).at(j)
    }
}
