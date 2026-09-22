use super::super::D1;
use super::At;
use alloc::vec::Vec;

impl<T: Copy> At<D1, T> for Vec<T> {
    fn at(&self, idx: usize) -> T {
        self[idx]
    }
}

impl<'a, T> At<D1, &'a T> for &'a Vec<T> {
    fn at(&self, idx: usize) -> &'a T {
        &self[idx]
    }
}
