use super::super::{D1, Dim, NVec};

impl<'a, T> NVec<D1, &'a mut T> for &'a mut [T] {
    fn at(&self, idx: impl Into<<D1 as Dim>::Idx>) -> &'a mut T {
        // &self[idx.into()]
        todo!()
    }
}
