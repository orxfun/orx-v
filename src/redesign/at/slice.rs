use super::super::D1;
use super::At;

// d1

impl<T: Copy> At<D1, T> for &[T] {
    fn at(&self, idx: usize) -> T {
        self[idx]
    }
}

impl<'a, T> At<D1, &'a T> for &'a [T] {
    fn at(&self, idx: usize) -> &'a T {
        &self[idx]
    }
}
