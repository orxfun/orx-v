use super::super::{D1, D2};
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

// d2

impl<'a, T, C1> At<D2, &'a T> for &'a [C1]
where
    &'a C1: At<D1, &'a T>,
{
    fn at(&self, [i, j]: [usize; 2]) -> &'a T {
        (&self[i]).at(j)
    }
}
