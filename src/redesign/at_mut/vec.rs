use super::super::{D1, D2};
use super::AtMut;
use alloc::vec::Vec;

// d1

impl<'a, T> AtMut<D1, T> for Vec<T> {
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
}

impl<T> AtMut<D1, T> for &mut Vec<T> {
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
}

// d2

impl<'a, T, C1> AtMut<D2, T> for Vec<C1>
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
}

impl<T, C1> AtMut<D2, T> for &mut Vec<C1>
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
}
