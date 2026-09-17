use super::super::{D1, Dim, NVec};
use alloc::vec::Vec;

impl<T: Copy> NVec<D1, T> for Vec<T> {
    fn at(&self, idx: <D1 as Dim>::Idx) -> T {
        self[idx]
    }
}
