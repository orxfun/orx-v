use super::super::{D1, D2, Dim, NVec, NVecMut, NVecRef};
use alloc::vec::Vec;

impl<T, V: NVecRef<D1, T>> NVecRef<D2, T> for Vec<V> {
    fn at_ref(&self, [i, j]: <D2 as Dim>::Idx) -> &T {
        self[i].at_ref(j)
    }

    // child

    type Child = V;

    fn child(&self, child_idx: <D2 as Dim>::ChildIdx) -> &Self::Child {
        &self[child_idx]
    }
}

impl<T, V: NVecMut<D1, T>> NVecMut<D2, T> for Vec<V> {
    fn at_mut(&mut self, [i, j]: <D2 as Dim>::Idx) -> &mut T {
        self[i].at_mut(j)
    }
}

impl<'b, T, V> NVec<D2, &'b T> for &'b Vec<V>
where
    for<'a> &'a V: NVec<D1, &'a T>,
{
    fn at<'a>(&'a self, [i, j]: <D2 as Dim>::Idx) -> &'b T
    where
        &'b T: 'a,
    {
        let row: &'b V = &(*self)[i];

        <&'b V as NVec<D1, &'b T>>::at(&row, j)
    }
}
