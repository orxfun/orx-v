use super::super::{D1, Dim, NVec, NVecRef};

impl<T> NVecRef<D1, T> for &'_ [T] {
    fn at_ref(&self, idx: <D1 as Dim>::Idx) -> &T {
        &self[idx]
    }
}

impl<'a, T> NVec<D1, &'a T> for &'a &'_ [T] {
    #[inline(always)]
    fn at(&self, idx: <D1 as Dim>::Idx) -> &'a T {
        <&'_ [T] as NVecRef<D1, T>>::at_ref(self, idx)
    }
}
