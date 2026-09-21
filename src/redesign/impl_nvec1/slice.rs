use super::super::{D1, Dim, NVec, NVecNever, NVecRef};

// ref

impl<T> NVecRef<D1, T> for &'_ [T] {
    fn at_ref(&self, idx: <D1 as Dim>::Idx) -> &T {
        &self[idx]
    }

    // child

    type Child = NVecNever;

    fn child(&self, _: <D1 as Dim>::ChildIdx) -> &Self::Child {
        unreachable!()
    }
}

// nvec

impl<'a, T> NVec<D1, &'a T> for &'a &'_ [T] {
    #[inline(always)]
    fn at<'r>(&'r self, idx: <D1 as Dim>::Idx) -> &'a T
    where
        &'a T: 'r,
    {
        <&'_ [T] as NVecRef<D1, T>>::at_ref(self, idx)
    }
}
