use super::super::{D1, Dim, NVecRef};

impl<T> NVecRef<D1, T> for &'_ [T] {
    fn at_ref(&self, idx: <D1 as Dim>::Idx) -> &T {
        &self[idx]
    }
}
