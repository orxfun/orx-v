use super::super::D1;
use super::MutAt;
use alloc::vec::Vec;

// d1

impl<'a, T> MutAt<D1, T> for Vec<T> {
    fn mut_at(&mut self, idx: usize) -> &mut T {
        &mut self[idx]
    }
}

impl<'a, T> MutAt<D1, T> for &'a mut Vec<T> {
    fn mut_at(&mut self, idx: usize) -> &mut T {
        &mut self[idx]
    }
}
