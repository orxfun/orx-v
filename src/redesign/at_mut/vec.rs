use super::super::{D1, D2, IdxNever};
use super::{AtMut, AtMutNever};
use alloc::vec::Vec;

// d1

impl<T> AtMut<D1, T> for Vec<T> {
    fn at(&self, idx: usize) -> &T {
        &self[idx]
    }

    fn try_at(&self, idx: usize) -> Option<&T> {
        self.get(idx)
    }

    fn at_mut(&mut self, idx: usize) -> &mut T {
        &mut self[idx]
    }

    fn try_at_mut(&mut self, idx: usize) -> Option<&mut T> {
        self.get_mut(idx)
    }

    type ChildMut<'c>
        = AtMutNever
    where
        Self: 'c;

    fn child_mut<'c>(&'c mut self, _: IdxNever) -> Self::ChildMut<'c> {
        unreachable!()
    }

    fn try_child_mut<'c>(&'c mut self, _: IdxNever) -> Option<Self::ChildMut<'c>> {
        unreachable!()
    }
}

// d2

impl<T, C1> AtMut<D2, T> for Vec<C1>
where
    C1: AtMut<D1, T>,
{
    fn at(&self, [i, j]: [usize; 2]) -> &T {
        self[i].at(j)
    }

    fn try_at(&self, [i, j]: [usize; 2]) -> Option<&T> {
        self.get(i).and_then(|x| x.try_at(j))
    }

    fn at_mut(&mut self, [i, j]: [usize; 2]) -> &mut T {
        self[i].at_mut(j)
    }

    fn try_at_mut(&mut self, [i, j]: [usize; 2]) -> Option<&mut T> {
        self.get_mut(i).and_then(|x| x.try_at_mut(j))
    }

    type ChildMut<'c>
        = &'c mut C1
    where
        Self: 'c;

    fn child_mut<'c>(&'c mut self, c: usize) -> Self::ChildMut<'c> {
        &mut self[c]
    }

    fn try_child_mut<'c>(&'c mut self, c: usize) -> Option<Self::ChildMut<'c>> {
        self.get_mut(c)
    }
}
