use super::super::{D1, D2, Dim, NVec, NVecRef};

impl<'b, T, V> NVecRef<D2, T> for &'b [V]
where
    V: NVecRef<D1, T>,
{
    #[inline(always)]
    fn at_ref(&self, [i, j]: <D2 as Dim>::Idx) -> &T {
        self[i].at_ref(j)
    }

    type Child = V;

    #[inline(always)]
    fn child(&self, child_idx: <D2 as Dim>::ChildIdx) -> &Self::Child {
        &self[child_idx]
    }
}

impl<'b, T, V> NVec<D2, &'b T> for &'b [V]
where
    for<'a> &'a V: NVec<D1, &'a T>,
{
    #[inline(always)]
    fn at<'r>(&'r self, [i, j]: <D2 as Dim>::Idx) -> &'b T
    where
        &'b T: 'r,
    {
        (&self[i]).at(j)
    }
}
