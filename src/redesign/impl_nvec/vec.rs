use super::super::{D1, Dim, NVec};
use alloc::vec::Vec;

impl<T> NVec<D1, T> for Vec<T> {
    type ItemRef<'a>
        = &'a T
    where
        Self: 'a;

    fn at(&self, idx: impl Into<<D1 as Dim>::Idx>) -> Self::ItemRef<'_> {
        &self[idx.into()]
    }
}
