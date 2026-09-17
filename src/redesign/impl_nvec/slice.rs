use super::super::{D1, Dim, NVec};

impl<'a, T> NVec<D1, &'a T> for &'a [T] {
    fn at(&self, idx: <D1 as Dim>::Idx) -> &'a T {
        &self[idx]
    }
}
