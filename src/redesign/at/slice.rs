use super::super::{D1, D2};
use super::At;

// d1

impl<'a, T> At<D1, &'a T> for &'a [T] {
    fn at(&self, idx: usize) -> &'a T {
        &self[idx]
    }

    fn try_at(&self, idx: usize) -> Option<&'a T> {
        self.get(idx)
    }
}

impl<T: Copy> At<D1, T> for &[T] {
    fn at(&self, idx: usize) -> T {
        self[idx]
    }

    fn try_at(&self, idx: usize) -> Option<T> {
        self.get(idx).copied()
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

    fn try_at(&self, [i, j]: [usize; 2]) -> Option<&'a T> {
        self.get(i).and_then(|x| x.try_at(j))
    }
}
