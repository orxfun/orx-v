use super::super::{D1, D2, IdxNever};
use super::{At, AtNever};
use alloc::vec::Vec;

// d1

impl<'a, T> At<D1, &'a T> for &'a Vec<T> {
    fn at(&self, idx: usize) -> &'a T {
        &self[idx]
    }

    fn try_at(&self, idx: usize) -> Option<&'a T> {
        self.get(idx)
    }

    type Child<'c>
        = AtNever
    where
        Self: 'c;

    fn child<'c>(&'c self, _: IdxNever) -> Self::Child<'c> {
        unreachable!()
    }

    fn try_child<'c>(&'c self, _: IdxNever) -> Option<Self::Child<'c>> {
        unreachable!()
    }
}

impl<T: Copy> At<D1, T> for Vec<T> {
    fn at(&self, idx: usize) -> T {
        self[idx]
    }

    fn try_at(&self, idx: usize) -> Option<T> {
        self.get(idx).copied()
    }

    type Child<'c>
        = AtNever
    where
        Self: 'c;

    fn child<'c>(&'c self, _: IdxNever) -> Self::Child<'c> {
        unreachable!()
    }

    fn try_child<'c>(&'c self, _: IdxNever) -> Option<Self::Child<'c>> {
        unreachable!()
    }
}

impl<T: Copy> At<D1, T> for &Vec<T> {
    fn at(&self, idx: usize) -> T {
        self[idx]
    }

    fn try_at(&self, idx: usize) -> Option<T> {
        self.get(idx).copied()
    }

    type Child<'c>
        = AtNever
    where
        Self: 'c;

    fn child<'c>(&'c self, _: IdxNever) -> Self::Child<'c> {
        unreachable!()
    }

    fn try_child<'c>(&'c self, _: IdxNever) -> Option<Self::Child<'c>> {
        unreachable!()
    }
}

// d2

impl<'a, T, C1> At<D2, &'a T> for &'a Vec<C1>
where
    &'a C1: At<D1, &'a T>,
{
    fn at(&self, [i, j]: [usize; 2]) -> &'a T {
        (&self[i]).at(j)
    }

    fn try_at(&self, [i, j]: [usize; 2]) -> Option<&'a T> {
        self.get(i).and_then(|x| x.try_at(j))
    }

    type Child<'c>
        = &'a C1
    where
        Self: 'c;

    fn child<'c>(&'c self, c: usize) -> Self::Child<'c> {
        &self[c]
    }

    fn try_child<'c>(&'c self, c: usize) -> Option<Self::Child<'c>> {
        self.get(c)
    }
}

impl<T, C1> At<D2, T> for Vec<C1>
where
    C1: At<D1, T>,
    for<'a> &'a C1: At<D1, T>,
{
    fn at(&self, [i, j]: [usize; 2]) -> T {
        self[i].at(j)
    }

    fn try_at(&self, [i, j]: [usize; 2]) -> Option<T> {
        self.get(i).and_then(|x| x.try_at(j))
    }

    type Child<'c>
        = &'c C1
    where
        Self: 'c;

    fn child<'c>(&'c self, c: usize) -> Self::Child<'c> {
        &self[c]
    }

    fn try_child<'c>(&'c self, c: usize) -> Option<Self::Child<'c>> {
        self.get(c)
    }
}
