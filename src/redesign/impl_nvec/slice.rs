use super::super::{D1, Dim, NVec};

impl<'a, T> NVec<D1, &'a T> for &'a [T] {
    type ItemRef<'b>
        = &'a T
    where
        Self: 'b;

    fn at(&self, idx: impl Into<<D1 as Dim>::Idx>) -> Self::ItemRef<'_> {
        &self[idx.into()]
    }
}
