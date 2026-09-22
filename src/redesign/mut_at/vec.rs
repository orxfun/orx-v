use super::super::{D1, D2};
use super::MutAt;
use alloc::vec::Vec;

// d1

impl<'a, T> MutAt<D1, T> for Vec<T> {
    fn at(&self, idx: usize) -> &T {
        &self[idx]
    }

    fn mut_at(&mut self, idx: usize) -> &mut T {
        &mut self[idx]
    }
}

impl<T> MutAt<D1, T> for &mut Vec<T> {
    fn at(&self, idx: usize) -> &T {
        &self[idx]
    }

    fn mut_at(&mut self, idx: usize) -> &mut T {
        &mut self[idx]
    }
}

// d2

impl<'a, T, C1> MutAt<D2, T> for Vec<C1>
where
    C1: MutAt<D1, T>,
{
    fn at(&self, [i, j]: [usize; 2]) -> &T {
        self[i].at(j)
    }

    fn mut_at(&mut self, [i, j]: [usize; 2]) -> &mut T {
        self[i].mut_at(j)
    }
}

impl<T, C1> MutAt<D2, T> for &mut Vec<C1>
where
    C1: MutAt<D1, T>,
{
    fn at(&self, [i, j]: [usize; 2]) -> &T {
        self[i].at(j)
    }

    fn mut_at(&mut self, [i, j]: [usize; 2]) -> &mut T {
        self[i].mut_at(j)
    }
}
