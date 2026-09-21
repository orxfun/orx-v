use super::super::{D1, Dim, NVec, NVecMut, NVecNever, NVecRef};
use alloc::vec::Vec;

// ref

impl<T> NVecRef<D1, T> for Vec<T> {
    #[inline(always)]
    fn at_ref(&self, idx: <D1 as Dim>::Idx) -> &T {
        &self[idx]
    }

    // child

    type Child = NVecNever;

    fn child(&self, _: <D1 as Dim>::ChildIdx) -> &Self::Child {
        unreachable!()
    }
}

impl<T> NVecRef<D1, T> for &Vec<T> {
    #[inline(always)]
    fn at_ref(&self, idx: <D1 as Dim>::Idx) -> &T {
        &self[idx]
    }

    // child

    type Child = NVecNever;

    fn child(&self, _: <D1 as Dim>::ChildIdx) -> &Self::Child {
        unreachable!()
    }
}

impl<T> NVecRef<D1, T> for &mut Vec<T> {
    #[inline(always)]
    fn at_ref(&self, idx: <D1 as Dim>::Idx) -> &T {
        &self[idx]
    }

    // child

    type Child = NVecNever;

    fn child(&self, _: <D1 as Dim>::ChildIdx) -> &Self::Child {
        unreachable!()
    }
}

// mut

impl<T> NVecMut<D1, T> for Vec<T> {
    #[inline(always)]
    fn at_mut(&mut self, idx: <D1 as Dim>::Idx) -> &mut T {
        &mut self[idx]
    }
}

impl<T> NVecMut<D1, T> for &mut Vec<T> {
    #[inline(always)]
    fn at_mut(&mut self, idx: <D1 as Dim>::Idx) -> &mut T {
        &mut self[idx]
    }
}

// nvec

impl<'a, T> NVec<D1, &'a T> for &'a Vec<T> {
    #[inline(always)]
    fn at<'r>(&'r self, idx: <D1 as Dim>::Idx) -> &'a T
    where
        &'a T: 'r,
    {
        <Vec<T> as NVecRef<D1, T>>::at_ref(self, idx)
    }
}
