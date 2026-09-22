use super::super::D1;
use super::MutAt;

// d1

impl<'a, T> MutAt<D1, T> for &'a mut [T] {
    fn at(&self, idx: usize) -> &T {
        &self[idx]
    }

    fn mut_at(&mut self, idx: usize) -> &mut T {
        &mut self[idx]
    }
}
