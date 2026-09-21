use super::{DNever, Dim, NVec, NVecMut, NVecRef};

pub struct NVecNever;

impl<T> NVecRef<DNever, T> for NVecNever {
    fn at_ref(&self, idx: <DNever as Dim>::Idx) -> &T {
        unreachable!()
    }

    type Child = Self;

    fn child(&self, child_idx: <DNever as Dim>::ChildIdx) -> &Self::Child {
        unreachable!()
    }
}

impl<T> NVecMut<DNever, T> for NVecNever {
    fn at_mut(&mut self, _: <DNever as Dim>::Idx) -> &mut T {
        unreachable!()
    }
}

impl<T> NVec<DNever, T> for NVecNever {
    fn at<'r>(&'r self, _: <DNever as Dim>::Idx) -> T
    where
        T: 'r,
    {
        unreachable!()
    }
}
