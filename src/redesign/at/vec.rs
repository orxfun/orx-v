use super::super::{D1, D2};
use super::At;
use alloc::vec::Vec;

// d1

impl<'a, T> At<D1, &'a T> for &'a Vec<T> {
    fn at(&self, idx: usize) -> &'a T {
        &self[idx]
    }
}

impl<T: Copy> At<D1, T> for Vec<T> {
    fn at(&self, idx: usize) -> T {
        self[idx]
    }
}

// d2

impl<'a, T, C1> At<D2, &'a T> for &'a Vec<C1>
where
    C1: At<D1, &'a T>,
{
    fn at(&self, [i, j]: [usize; 2]) -> &'a T {
        self[i].at(j)
    }
}

impl<T, C1> At<D2, T> for Vec<C1>
where
    C1: At<D1, T>,
{
    fn at(&self, [i, j]: [usize; 2]) -> T {
        self[i].at(j)
    }
}
